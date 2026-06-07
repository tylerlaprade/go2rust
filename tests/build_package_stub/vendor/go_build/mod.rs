use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::gc::*;
use crate::read::*;
use crate::zcgo::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const FIND_ONLY: u64 = 1 << 0;
pub const ALLOW_BINARY: u64 = 1 << 1;
pub const IMPORT_COMMENT: u64 = 1 << 2;
pub const IGNORE_VENDOR: u64 = 1 << 3;


pub(crate) const SAFE_STRING: &'static str = "+-.,/0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz:$@%! ~^";


/// A Context specifies the supporting context for a build.
#[derive(Clone)]
pub struct Context {
    pub g_o_a_r_c_h: Arc<Mutex<Option<String>>>,
    pub g_o_o_s: Arc<Mutex<Option<String>>>,
    pub g_o_r_o_o_t: Arc<Mutex<Option<String>>>,
    pub g_o_p_a_t_h: Arc<Mutex<Option<String>>>,
    pub dir: Arc<Mutex<Option<String>>>,
    pub cgo_enabled: Arc<Mutex<Option<bool>>>,
    pub use_all_files: Arc<Mutex<Option<bool>>>,
    pub compiler: Arc<Mutex<Option<String>>>,
    pub build_tags: Arc<Mutex<Option<Vec<String>>>>,
    pub tool_tags: Arc<Mutex<Option<Vec<String>>>>,
    pub release_tags: Arc<Mutex<Option<Vec<String>>>>,
    pub install_suffix: Arc<Mutex<Option<String>>>,
    pub join_path: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>,
    pub split_path_list: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> + Send + Sync>>>>,
    pub is_abs_path: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>,
    pub is_dir: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>>>>,
    pub has_subdir: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) + Send + Sync>>>>,
    pub read_dir: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<fs_FileInfo>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>,
    pub open_file: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<io_ReadCloser>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>,
}

impl Context {
    pub fn __go_value_clone(&self) -> Self {
        Self { g_o_a_r_c_h: { let __guard = self.g_o_a_r_c_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_o_o_s: { let __guard = self.g_o_o_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_o_r_o_o_t: { let __guard = self.g_o_r_o_o_t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_o_p_a_t_h: { let __guard = self.g_o_p_a_t_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cgo_enabled: { let __guard = self.cgo_enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, use_all_files: { let __guard = self.use_all_files.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, compiler: { let __guard = self.compiler.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, build_tags: self.build_tags.clone(), tool_tags: self.tool_tags.clone(), release_tags: self.release_tags.clone(), install_suffix: { let __guard = self.install_suffix.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, join_path: self.join_path.clone(), split_path_list: self.split_path_list.clone(), is_abs_path: self.is_abs_path.clone(), is_dir: self.is_dir.clone(), has_subdir: self.has_subdir.clone(), read_dir: self.read_dir.clone(), open_file: self.open_file.clone() }
    }
}


impl Default for Context {
    fn default() -> Self {
        Self { g_o_a_r_c_h: Arc::new(Mutex::new(Some(String::new()))), g_o_o_s: Arc::new(Mutex::new(Some(String::new()))), g_o_r_o_o_t: Arc::new(Mutex::new(Some(String::new()))), g_o_p_a_t_h: Arc::new(Mutex::new(Some(String::new()))), dir: Arc::new(Mutex::new(Some(String::new()))), cgo_enabled: Arc::new(Mutex::new(Some(false))), use_all_files: Arc::new(Mutex::new(Some(false))), compiler: Arc::new(Mutex::new(Some(String::new()))), build_tags: Arc::new(Mutex::new(None)), tool_tags: Arc::new(Mutex::new(None)), release_tags: Arc::new(Mutex::new(None)), install_suffix: Arc::new(Mutex::new(Some(String::new()))), join_path: Arc::new(Mutex::new(None)), split_path_list: Arc::new(Mutex::new(None)), is_abs_path: Arc::new(Mutex::new(None)), is_dir: Arc::new(Mutex::new(None)), has_subdir: Arc::new(Mutex::new(None)), read_dir: Arc::new(Mutex::new(None)), open_file: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.g_o_a_r_c_h.lock().unwrap().as_ref().unwrap()), (*self.g_o_o_s.lock().unwrap().as_ref().unwrap()), (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()), (*self.g_o_p_a_t_h.lock().unwrap().as_ref().unwrap()), (*self.dir.lock().unwrap().as_ref().unwrap()), (*self.cgo_enabled.lock().unwrap().as_ref().unwrap()), (*self.use_all_files.lock().unwrap().as_ref().unwrap()), (*self.compiler.lock().unwrap().as_ref().unwrap()), format_slice(&self.build_tags), format_slice(&self.tool_tags), format_slice(&self.release_tags), (*self.install_suffix.lock().unwrap().as_ref().unwrap()), "<func>", "<func>", "<func>", "<func>", "<func>", "<func>", "<func>")
    }
}

impl GoJsonDecode for Context {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("GOARCH") {
            out.g_o_a_r_c_h = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GOOS") {
            out.g_o_o_s = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GOROOT") {
            out.g_o_r_o_o_t = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GOPATH") {
            out.g_o_p_a_t_h = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Dir") {
            out.dir = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoEnabled") {
            out.cgo_enabled = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("UseAllFiles") {
            out.use_all_files = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Compiler") {
            out.compiler = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("BuildTags") {
            out.build_tags = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ToolTags") {
            out.tool_tags = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ReleaseTags") {
            out.release_tags = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("InstallSuffix") {
            out.install_suffix = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// An ImportMode controls the behavior of the Import method.
#[derive(Debug, Clone, Default)]
pub struct ImportMode(pub Arc<Mutex<Option<u64>>>);

impl Display for ImportMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ImportMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for ImportMode {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ImportMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for ImportMode {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ImportMode> for u64 {
    fn eq(&self, other: &ImportMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ImportMode> for u64 {
    fn partial_cmp(&self, other: &ImportMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ImportMode {
    type Output = ImportMode;
    fn add(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for ImportMode {
    type Output = ImportMode;
    fn add(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ImportMode> for u64 {
    type Output = ImportMode;
    fn add(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ImportMode {
    type Output = ImportMode;
    fn sub(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for ImportMode {
    type Output = ImportMode;
    fn sub(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ImportMode> for u64 {
    type Output = ImportMode;
    fn sub(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ImportMode {
    type Output = ImportMode;
    fn mul(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for ImportMode {
    type Output = ImportMode;
    fn mul(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ImportMode> for u64 {
    type Output = ImportMode;
    fn mul(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ImportMode {
    type Output = ImportMode;
    fn div(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for ImportMode {
    type Output = ImportMode;
    fn div(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ImportMode> for u64 {
    type Output = ImportMode;
    fn div(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ImportMode {
    type Output = ImportMode;
    fn rem(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for ImportMode {
    type Output = ImportMode;
    fn rem(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ImportMode> for u64 {
    type Output = ImportMode;
    fn rem(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ImportMode {
    type Output = ImportMode;
    fn bitand(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for ImportMode {
    type Output = ImportMode;
    fn bitand(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ImportMode> for u64 {
    type Output = ImportMode;
    fn bitand(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ImportMode {
    type Output = ImportMode;
    fn bitor(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for ImportMode {
    type Output = ImportMode;
    fn bitor(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ImportMode> for u64 {
    type Output = ImportMode;
    fn bitor(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ImportMode {
    type Output = ImportMode;
    fn bitxor(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for ImportMode {
    type Output = ImportMode;
    fn bitxor(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ImportMode> for u64 {
    type Output = ImportMode;
    fn bitxor(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ImportMode {
    type Output = ImportMode;
    fn not(self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: i8) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: i16) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: i64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: u32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: u8) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: u16) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ImportMode {
    type Output = ImportMode;
    fn shl(self, other: usize) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: i8) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: i16) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: i64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: u32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: u8) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: u16) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: u64) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ImportMode {
    type Output = ImportMode;
    fn shr(self, other: usize) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ImportMode {}

impl Ord for ImportMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Package describes the Go package found in a directory.
#[derive(Clone)]
pub struct Package {
    pub dir: Arc<Mutex<Option<String>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub import_comment: Arc<Mutex<Option<String>>>,
    pub doc: Arc<Mutex<Option<String>>>,
    pub import_path: Arc<Mutex<Option<String>>>,
    pub root: Arc<Mutex<Option<String>>>,
    pub src_root: Arc<Mutex<Option<String>>>,
    pub pkg_root: Arc<Mutex<Option<String>>>,
    pub pkg_target_root: Arc<Mutex<Option<String>>>,
    pub bin_dir: Arc<Mutex<Option<String>>>,
    pub goroot: Arc<Mutex<Option<bool>>>,
    pub pkg_obj: Arc<Mutex<Option<String>>>,
    pub all_tags: Arc<Mutex<Option<Vec<String>>>>,
    pub conflict_dir: Arc<Mutex<Option<String>>>,
    pub binary_only: Arc<Mutex<Option<bool>>>,
    pub go_files: Arc<Mutex<Option<Vec<String>>>>,
    pub cgo_files: Arc<Mutex<Option<Vec<String>>>>,
    pub ignored_go_files: Arc<Mutex<Option<Vec<String>>>>,
    pub invalid_go_files: Arc<Mutex<Option<Vec<String>>>>,
    pub ignored_other_files: Arc<Mutex<Option<Vec<String>>>>,
    pub c_files: Arc<Mutex<Option<Vec<String>>>>,
    pub c_x_x_files: Arc<Mutex<Option<Vec<String>>>>,
    pub m_files: Arc<Mutex<Option<Vec<String>>>>,
    pub h_files: Arc<Mutex<Option<Vec<String>>>>,
    pub f_files: Arc<Mutex<Option<Vec<String>>>>,
    pub s_files: Arc<Mutex<Option<Vec<String>>>>,
    pub swig_files: Arc<Mutex<Option<Vec<String>>>>,
    pub swig_c_x_x_files: Arc<Mutex<Option<Vec<String>>>>,
    pub syso_files: Arc<Mutex<Option<Vec<String>>>>,
    pub cgo_c_f_l_a_g_s: Arc<Mutex<Option<Vec<String>>>>,
    pub cgo_c_p_p_f_l_a_g_s: Arc<Mutex<Option<Vec<String>>>>,
    pub cgo_c_x_x_f_l_a_g_s: Arc<Mutex<Option<Vec<String>>>>,
    pub cgo_f_f_l_a_g_s: Arc<Mutex<Option<Vec<String>>>>,
    pub cgo_l_d_f_l_a_g_s: Arc<Mutex<Option<Vec<String>>>>,
    pub cgo_pkg_config: Arc<Mutex<Option<Vec<String>>>>,
    pub test_go_files: Arc<Mutex<Option<Vec<String>>>>,
    pub x_test_go_files: Arc<Mutex<Option<Vec<String>>>>,
    pub directives: Arc<Mutex<Option<Vec<Directive>>>>,
    pub test_directives: Arc<Mutex<Option<Vec<Directive>>>>,
    pub x_test_directives: Arc<Mutex<Option<Vec<Directive>>>>,
    pub imports: Arc<Mutex<Option<Vec<String>>>>,
    pub import_pos: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>,
    pub test_imports: Arc<Mutex<Option<Vec<String>>>>,
    pub test_import_pos: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>,
    pub x_test_imports: Arc<Mutex<Option<Vec<String>>>>,
    pub x_test_import_pos: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>,
    pub embed_patterns: Arc<Mutex<Option<Vec<String>>>>,
    pub embed_pattern_pos: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>,
    pub test_embed_patterns: Arc<Mutex<Option<Vec<String>>>>,
    pub test_embed_pattern_pos: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>,
    pub x_test_embed_patterns: Arc<Mutex<Option<Vec<String>>>>,
    pub x_test_embed_pattern_pos: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, import_comment: { let __guard = self.import_comment.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, doc: { let __guard = self.doc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, import_path: { let __guard = self.import_path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, root: { let __guard = self.root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, src_root: { let __guard = self.src_root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkg_root: { let __guard = self.pkg_root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkg_target_root: { let __guard = self.pkg_target_root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bin_dir: { let __guard = self.bin_dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, goroot: { let __guard = self.goroot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkg_obj: { let __guard = self.pkg_obj.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, all_tags: self.all_tags.clone(), conflict_dir: { let __guard = self.conflict_dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, binary_only: { let __guard = self.binary_only.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go_files: self.go_files.clone(), cgo_files: self.cgo_files.clone(), ignored_go_files: self.ignored_go_files.clone(), invalid_go_files: self.invalid_go_files.clone(), ignored_other_files: self.ignored_other_files.clone(), c_files: self.c_files.clone(), c_x_x_files: self.c_x_x_files.clone(), m_files: self.m_files.clone(), h_files: self.h_files.clone(), f_files: self.f_files.clone(), s_files: self.s_files.clone(), swig_files: self.swig_files.clone(), swig_c_x_x_files: self.swig_c_x_x_files.clone(), syso_files: self.syso_files.clone(), cgo_c_f_l_a_g_s: self.cgo_c_f_l_a_g_s.clone(), cgo_c_p_p_f_l_a_g_s: self.cgo_c_p_p_f_l_a_g_s.clone(), cgo_c_x_x_f_l_a_g_s: self.cgo_c_x_x_f_l_a_g_s.clone(), cgo_f_f_l_a_g_s: self.cgo_f_f_l_a_g_s.clone(), cgo_l_d_f_l_a_g_s: self.cgo_l_d_f_l_a_g_s.clone(), cgo_pkg_config: self.cgo_pkg_config.clone(), test_go_files: self.test_go_files.clone(), x_test_go_files: self.x_test_go_files.clone(), directives: self.directives.clone(), test_directives: self.test_directives.clone(), x_test_directives: self.x_test_directives.clone(), imports: self.imports.clone(), import_pos: self.import_pos.clone(), test_imports: self.test_imports.clone(), test_import_pos: self.test_import_pos.clone(), x_test_imports: self.x_test_imports.clone(), x_test_import_pos: self.x_test_import_pos.clone(), embed_patterns: self.embed_patterns.clone(), embed_pattern_pos: self.embed_pattern_pos.clone(), test_embed_patterns: self.test_embed_patterns.clone(), test_embed_pattern_pos: self.test_embed_pattern_pos.clone(), x_test_embed_patterns: self.x_test_embed_patterns.clone(), x_test_embed_pattern_pos: self.x_test_embed_pattern_pos.clone() }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { dir: Arc::new(Mutex::new(Some(String::new()))), name: Arc::new(Mutex::new(Some(String::new()))), import_comment: Arc::new(Mutex::new(Some(String::new()))), doc: Arc::new(Mutex::new(Some(String::new()))), import_path: Arc::new(Mutex::new(Some(String::new()))), root: Arc::new(Mutex::new(Some(String::new()))), src_root: Arc::new(Mutex::new(Some(String::new()))), pkg_root: Arc::new(Mutex::new(Some(String::new()))), pkg_target_root: Arc::new(Mutex::new(Some(String::new()))), bin_dir: Arc::new(Mutex::new(Some(String::new()))), goroot: Arc::new(Mutex::new(Some(false))), pkg_obj: Arc::new(Mutex::new(Some(String::new()))), all_tags: Arc::new(Mutex::new(None)), conflict_dir: Arc::new(Mutex::new(Some(String::new()))), binary_only: Arc::new(Mutex::new(Some(false))), go_files: Arc::new(Mutex::new(None)), cgo_files: Arc::new(Mutex::new(None)), ignored_go_files: Arc::new(Mutex::new(None)), invalid_go_files: Arc::new(Mutex::new(None)), ignored_other_files: Arc::new(Mutex::new(None)), c_files: Arc::new(Mutex::new(None)), c_x_x_files: Arc::new(Mutex::new(None)), m_files: Arc::new(Mutex::new(None)), h_files: Arc::new(Mutex::new(None)), f_files: Arc::new(Mutex::new(None)), s_files: Arc::new(Mutex::new(None)), swig_files: Arc::new(Mutex::new(None)), swig_c_x_x_files: Arc::new(Mutex::new(None)), syso_files: Arc::new(Mutex::new(None)), cgo_c_f_l_a_g_s: Arc::new(Mutex::new(None)), cgo_c_p_p_f_l_a_g_s: Arc::new(Mutex::new(None)), cgo_c_x_x_f_l_a_g_s: Arc::new(Mutex::new(None)), cgo_f_f_l_a_g_s: Arc::new(Mutex::new(None)), cgo_l_d_f_l_a_g_s: Arc::new(Mutex::new(None)), cgo_pkg_config: Arc::new(Mutex::new(None)), test_go_files: Arc::new(Mutex::new(None)), x_test_go_files: Arc::new(Mutex::new(None)), directives: Arc::new(Mutex::new(None)), test_directives: Arc::new(Mutex::new(None)), x_test_directives: Arc::new(Mutex::new(None)), imports: Arc::new(Mutex::new(None)), import_pos: Arc::new(Mutex::new(None)), test_imports: Arc::new(Mutex::new(None)), test_import_pos: Arc::new(Mutex::new(None)), x_test_imports: Arc::new(Mutex::new(None)), x_test_import_pos: Arc::new(Mutex::new(None)), embed_patterns: Arc::new(Mutex::new(None)), embed_pattern_pos: Arc::new(Mutex::new(None)), test_embed_patterns: Arc::new(Mutex::new(None)), test_embed_pattern_pos: Arc::new(Mutex::new(None)), x_test_embed_patterns: Arc::new(Mutex::new(None)), x_test_embed_pattern_pos: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.dir.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()), (*self.import_comment.lock().unwrap().as_ref().unwrap()), (*self.doc.lock().unwrap().as_ref().unwrap()), (*self.import_path.lock().unwrap().as_ref().unwrap()), (*self.root.lock().unwrap().as_ref().unwrap()), (*self.src_root.lock().unwrap().as_ref().unwrap()), (*self.pkg_root.lock().unwrap().as_ref().unwrap()), (*self.pkg_target_root.lock().unwrap().as_ref().unwrap()), (*self.bin_dir.lock().unwrap().as_ref().unwrap()), (*self.goroot.lock().unwrap().as_ref().unwrap()), (*self.pkg_obj.lock().unwrap().as_ref().unwrap()), format_slice(&self.all_tags), (*self.conflict_dir.lock().unwrap().as_ref().unwrap()), (*self.binary_only.lock().unwrap().as_ref().unwrap()), format_slice(&self.go_files), format_slice(&self.cgo_files), format_slice(&self.ignored_go_files), format_slice(&self.invalid_go_files), format_slice(&self.ignored_other_files), format_slice(&self.c_files), format_slice(&self.c_x_x_files), format_slice(&self.m_files), format_slice(&self.h_files), format_slice(&self.f_files), format_slice(&self.s_files), format_slice(&self.swig_files), format_slice(&self.swig_c_x_x_files), format_slice(&self.syso_files), format_slice(&self.cgo_c_f_l_a_g_s), format_slice(&self.cgo_c_p_p_f_l_a_g_s), format_slice(&self.cgo_c_x_x_f_l_a_g_s), format_slice(&self.cgo_f_f_l_a_g_s), format_slice(&self.cgo_l_d_f_l_a_g_s), format_slice(&self.cgo_pkg_config), format_slice(&self.test_go_files), format_slice(&self.x_test_go_files), format_slice(&self.directives), format_slice(&self.test_directives), format_slice(&self.x_test_directives), format_slice(&self.imports), "<map>", format_slice(&self.test_imports), "<map>", format_slice(&self.x_test_imports), "<map>", format_slice(&self.embed_patterns), "<map>", format_slice(&self.test_embed_patterns), "<map>", format_slice(&self.x_test_embed_patterns), "<map>")
    }
}

impl GoJsonDecode for Package {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Dir") {
            out.dir = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ImportComment") {
            out.import_comment = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Doc") {
            out.doc = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ImportPath") {
            out.import_path = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Root") {
            out.root = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SrcRoot") {
            out.src_root = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("PkgRoot") {
            out.pkg_root = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("PkgTargetRoot") {
            out.pkg_target_root = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("BinDir") {
            out.bin_dir = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Goroot") {
            out.goroot = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("PkgObj") {
            out.pkg_obj = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("AllTags") {
            out.all_tags = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ConflictDir") {
            out.conflict_dir = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("BinaryOnly") {
            out.binary_only = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GoFiles") {
            out.go_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoFiles") {
            out.cgo_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IgnoredGoFiles") {
            out.ignored_go_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("InvalidGoFiles") {
            out.invalid_go_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IgnoredOtherFiles") {
            out.ignored_other_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CFiles") {
            out.c_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CXXFiles") {
            out.c_x_x_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("MFiles") {
            out.m_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HFiles") {
            out.h_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("FFiles") {
            out.f_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SFiles") {
            out.s_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SwigFiles") {
            out.swig_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SwigCXXFiles") {
            out.swig_c_x_x_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SysoFiles") {
            out.syso_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoCFLAGS") {
            out.cgo_c_f_l_a_g_s = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoCPPFLAGS") {
            out.cgo_c_p_p_f_l_a_g_s = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoCXXFLAGS") {
            out.cgo_c_x_x_f_l_a_g_s = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoFFLAGS") {
            out.cgo_f_f_l_a_g_s = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoLDFLAGS") {
            out.cgo_l_d_f_l_a_g_s = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("CgoPkgConfig") {
            out.cgo_pkg_config = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("TestGoFiles") {
            out.test_go_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("XTestGoFiles") {
            out.x_test_go_files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Imports") {
            out.imports = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("TestImports") {
            out.test_imports = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("XTestImports") {
            out.x_test_imports = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("EmbedPatterns") {
            out.embed_patterns = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("TestEmbedPatterns") {
            out.test_embed_patterns = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("XTestEmbedPatterns") {
            out.x_test_embed_patterns = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A Directive is a Go directive comment (//go:zzz...) found in a source file.
#[derive(Clone)]
pub struct Directive {
    pub text: Arc<Mutex<Option<String>>>,
    pub pos: Arc<Mutex<Option<token_Position>>>,
}

impl Directive {
    pub fn __go_value_clone(&self) -> Self {
        Self { text: { let __guard = self.text.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Directive {
    fn default() -> Self {
        Self { text: Arc::new(Mutex::new(Some(String::new()))), pos: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.text.lock().unwrap().as_ref().unwrap()), (*self.pos.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Directive {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Text") {
            out.text = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// NoGoError is the error used by [Import] to describe a directory
/// containing no buildable Go source files. (It may still contain
/// test files, files hidden by build tags, and so on.)
#[derive(Debug, Clone)]
pub struct NoGoError {
    pub dir: Arc<Mutex<Option<String>>>,
}

impl NoGoError {
    pub fn __go_value_clone(&self) -> Self {
        Self { dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for NoGoError {
    fn default() -> Self {
        Self { dir: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for NoGoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for NoGoError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Dir") {
            out.dir = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// MultiplePackageError describes a directory containing
/// multiple buildable Go source files for multiple packages.
#[derive(Debug, Clone)]
pub struct MultiplePackageError {
    pub dir: Arc<Mutex<Option<String>>>,
    pub packages: Arc<Mutex<Option<Vec<String>>>>,
    pub files: Arc<Mutex<Option<Vec<String>>>>,
}

impl MultiplePackageError {
    pub fn __go_value_clone(&self) -> Self {
        Self { dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, packages: self.packages.clone(), files: self.files.clone() }
    }
}


impl Default for MultiplePackageError {
    fn default() -> Self {
        Self { dir: Arc::new(Mutex::new(Some(String::new()))), packages: Arc::new(Mutex::new(None)), files: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for MultiplePackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for MultiplePackageError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Dir") {
            out.dir = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Packages") {
            out.packages = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Files") {
            out.files = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// fileInfo records information learned about a file included in a build.
#[derive(Clone)]
pub struct fileInfo {
    pub name: Arc<Mutex<Option<String>>>,
    pub header: Arc<Mutex<Option<Vec<u8>>>>,
    pub fset: Arc<Mutex<Option<token_FileSet>>>,
    pub parsed: Arc<Mutex<Option<ast_File>>>,
    pub parse_err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>,
    pub imports: Arc<Mutex<Option<Vec<fileImport>>>>,
    pub embeds: Arc<Mutex<Option<Vec<fileEmbed>>>>,
    pub directives: Arc<Mutex<Option<Vec<Directive>>>>,
}

impl fileInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, header: self.header.clone(), fset: self.fset.clone(), parsed: self.parsed.clone(), parse_err: self.parse_err.clone(), imports: self.imports.clone(), embeds: self.embeds.clone(), directives: self.directives.clone() }
    }
}


impl Default for fileInfo {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), header: Arc::new(Mutex::new(None)), fset: Arc::new(Mutex::new(None)), parsed: Arc::new(Mutex::new(None)), parse_err: Arc::new(Mutex::new(None)), imports: Arc::new(Mutex::new(None)), embeds: Arc::new(Mutex::new(None)), directives: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for fileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), format_slice(&self.header), { let __guard = self.fset.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.parsed.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.parse_err.lock().unwrap().as_ref().unwrap()), format_slice(&self.imports), format_slice(&self.embeds), format_slice(&self.directives))
    }
}

impl GoJsonDecode for fileInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct fileImport {
    pub path: Arc<Mutex<Option<String>>>,
    pub pos: Arc<Mutex<Option<token_Pos>>>,
    pub doc: Arc<Mutex<Option<ast_CommentGroup>>>,
}

impl fileImport {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, doc: self.doc.clone() }
    }
}


impl Default for fileImport {
    fn default() -> Self {
        Self { path: Arc::new(Mutex::new(Some(String::new()))), pos: Arc::new(Mutex::new(Some(token_Pos(0)))), doc: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for fileImport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.path.lock().unwrap().as_ref().unwrap()), (*self.pos.lock().unwrap().as_ref().unwrap()), { let __guard = self.doc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for fileImport {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct fileEmbed {
    pub pattern: Arc<Mutex<Option<String>>>,
    pub pos: Arc<Mutex<Option<token_Position>>>,
}

impl fileEmbed {
    pub fn __go_value_clone(&self) -> Self {
        Self { pattern: { let __guard = self.pattern.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for fileEmbed {
    fn default() -> Self {
        Self { pattern: Arc::new(Mutex::new(Some(String::new()))), pos: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for fileEmbed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.pattern.lock().unwrap().as_ref().unwrap()), (*self.pos.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for fileEmbed {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub static Default: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Context>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static defaultToolTags: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static defaultReleaseTags: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static installgoroot: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<internal_godebug::r#mod::Setting>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errNoModules: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static slashSlash: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static slashStar: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static starSlash: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static newline: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static dummyPkg: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Package>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static plusBuild: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static goBuildComment: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errMultipleGoBuild: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static binaryOnlyComment: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ToolDir: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Default.lock().unwrap() = Some(Default::default());
    *defaultToolTags.lock().unwrap() = Some(vec![]);
    *defaultReleaseTags.lock().unwrap() = Some(vec![]);
    *installgoroot.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *errNoModules.lock().unwrap() = None;
    *slashSlash.lock().unwrap() = Some(vec![]);
    *slashStar.lock().unwrap() = Some(vec![]);
    *starSlash.lock().unwrap() = Some(vec![]);
    *newline.lock().unwrap() = Some(vec![]);
    *dummyPkg.lock().unwrap() = Some(Default::default());
    *plusBuild.lock().unwrap() = Some(vec![]);
    *goBuildComment.lock().unwrap() = Some(vec![]);
    *errMultipleGoBuild.lock().unwrap() = None;
    *binaryOnlyComment.lock().unwrap() = Some(vec![]);
    *ToolDir.lock().unwrap() = Some(String::new());
    *Default.lock().unwrap() = Some((*default_context().lock().unwrap().as_ref().unwrap()).clone());
    *installgoroot.lock().unwrap() = Some(internal_godebug::new(Arc::new(Mutex::new(Some("installgoroot".to_string())))));
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("not using modules".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNoModules.lock().unwrap() = new_val; }
    *slashSlash.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("//".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
    *slashStar.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("/*".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
    *starSlash.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("*/".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
    *newline.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("\n".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
    *plusBuild.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("+build".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
    *goBuildComment.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("//go:build".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("multiple //go:build comments".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errMultipleGoBuild.lock().unwrap() = new_val; }
    *binaryOnlyComment.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("//go:binary-only-package".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
    *ToolDir.lock().unwrap() = Some((*get_tool_dir().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *Default.lock().unwrap() = Some(Default::default());
    *defaultToolTags.lock().unwrap() = Some(vec![]);
    *defaultReleaseTags.lock().unwrap() = Some(vec![]);
    *installgoroot.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *errNoModules.lock().unwrap() = None;
    *slashSlash.lock().unwrap() = Some(vec![]);
    *slashStar.lock().unwrap() = Some(vec![]);
    *starSlash.lock().unwrap() = Some(vec![]);
    *newline.lock().unwrap() = Some(vec![]);
    *dummyPkg.lock().unwrap() = Some(Default::default());
    *plusBuild.lock().unwrap() = Some(vec![]);
    *goBuildComment.lock().unwrap() = Some(vec![]);
    *errMultipleGoBuild.lock().unwrap() = None;
    *binaryOnlyComment.lock().unwrap() = Some(vec![]);
    *ToolDir.lock().unwrap() = Some(String::new());
}


pub(crate) fn __go_init_order_0() {
    *Default.lock().unwrap() = Some((*default_context().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_1() {
    *installgoroot.lock().unwrap() = Some(internal_godebug::new(Arc::new(Mutex::new(Some("installgoroot".to_string())))));
}


pub(crate) fn __go_init_order_2() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("not using modules".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNoModules.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_3() {
    *slashSlash.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("//".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_4() {
    *slashStar.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("/*".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_5() {
    *starSlash.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("*/".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_6() {
    *newline.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("\n".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_7() {
    *plusBuild.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("+build".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_8() {
    *goBuildComment.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("//go:build".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_9() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("multiple //go:build comments".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errMultipleGoBuild.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_10() {
    *binaryOnlyComment.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("//go:binary-only-package".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_11() {
    *ToolDir.lock().unwrap() = Some((*get_tool_dir().lock().unwrap().as_ref().unwrap()).clone());
}


impl Context {
    /// joinPath calls ctxt.JoinPath (if not nil) or else filepath.Join.
    pub fn join_path(&self, elem: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> {
        {
        let mut f = self.join_path.clone();;
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(elem.clone()) };;
        }
    }
        path_filepath::join(elem.clone())
    }

    /// splitPathList calls ctxt.SplitPathList (if not nil) or else filepath.SplitList.
    pub fn split_path_list(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        {
        let mut f = self.split_path_list.clone();;
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };;
        }
    }
        path_filepath::split_list(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// isAbsPath calls ctxt.IsAbsPath (if not nil) or else filepath.IsAbs.
    pub fn is_abs_path(&self, path: Arc<Mutex<Option<String>>>) -> bool {
        {
        let mut f = self.is_abs_path.clone();;
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };;
        }
    }
        path_filepath::is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// isDir calls ctxt.IsDir (if not nil) or else uses os.Stat.
    pub fn is_dir(&self, path: Arc<Mutex<Option<String>>>) -> bool {
        {
        let mut f = self.is_dir.clone();;
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };;
        }
    }
        let (mut fi, mut err) = os::stat({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        return { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && (*fi.lock().unwrap().as_ref().unwrap()).is_dir();
    }

    /// hasSubdir calls ctxt.HasSubdir (if not nil) or else uses
    /// the local file system to answer the question.
    pub fn has_subdir(&self, root: Arc<Mutex<Option<String>>>, dir: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
    let mut rel: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        {
        let mut f = self.has_subdir.clone();;
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };;
        }
    }
                // Try using paths we received.
        {
        { let (__tmp_0, __tmp_1) = has_subdir(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *rel.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };;
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            return (rel.clone(), (*ok.lock().unwrap().as_ref().unwrap()));;
        }
    }
                // Try expanding symlinks and comparing
                // expanded against unexpanded and
                // expanded against expanded.
        let (mut rootSym, _) = path_filepath::eval_symlinks(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let (mut dirSym, _) = path_filepath::eval_symlinks(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
        { let (__tmp_0, __tmp_1) = has_subdir(Arc::new(Mutex::new(Some({ let __arg_holder = rootSym.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *rel.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };;
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            return (rel.clone(), (*ok.lock().unwrap().as_ref().unwrap()));;
        }
    }
        {
        { let (__tmp_0, __tmp_1) = has_subdir(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dirSym.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *rel.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };;
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            return (rel.clone(), (*ok.lock().unwrap().as_ref().unwrap()));;
        }
    }
        return has_subdir(Arc::new(Mutex::new(Some({ let __arg_holder = rootSym.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dirSym.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// readDir calls ctxt.ReadDir (if not nil) or else os.ReadDir.
    pub fn read_dir(&self, path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<fs_DirEntry>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
                // TODO: add a fs.DirEntry version of Context.ReadDir
        {
        let mut f = self.read_dir.clone();;
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
            let (mut fis, mut err) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<fs_FileInfo>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<fs_FileInfo>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };;
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    };
            let mut des: Arc<Mutex<Option<Vec<fs_DirEntry>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ((*fis.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));;
            { let __range_holder = fis.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, fi) in __range_values.iter().enumerate() {
        (*des.lock().unwrap().as_mut().unwrap())[(i) as usize] = (*fs::file_info_to_dir_entry(fi.clone()).lock().unwrap().as_ref().unwrap()).clone();
    } };
            return (des.clone(), Arc::new(Mutex::new(None)));;
        }
    }
        os::read_dir({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })
    }

    /// openFile calls ctxt.OpenFile (if not nil) or else os.Open.
    pub fn open_file(&self, path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<io_ReadCloser>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        {
        let mut r#fn = self.open_file.clone();;
        if { let __nil_result = (*r#fn.lock().unwrap()).is_some(); __nil_result } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<io_ReadCloser>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<io_ReadCloser>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };;
        }
    }
        let (mut f, mut err) = os::open({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
                // nil interface
        return ({ let __arg = f.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<io_ReadCloser> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(None)));
    }

    /// isFile determines whether path is a file by trying to open it.
    /// It reuses openFile instead of adding another function to the
    /// list in Context.
    pub fn is_file(&self, path: Arc<Mutex<Option<String>>>) -> bool {
        let (mut f, mut err) = self.open_file(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return false;
    }
        (*f.lock().unwrap().as_ref().unwrap()).close();
        true
    }

    /// gopath returns the list of Go path directories.
    pub fn gopath(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut all: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.g_o_p_a_t_h.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.split_path_list(__method_arg0) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        if { let __tmp_x = (*p).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*p).clone(); let __tmp_y = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // Empty paths are uninteresting.
                // If the path is the GOROOT, ignore it.
                // People sometimes set GOPATH=$GOROOT.
                // Do not get confused by this common mistake.
        continue
    }
                // Empty paths are uninteresting.
                // If the path is the GOROOT, ignore it.
                // People sometimes set GOPATH=$GOROOT.
                // Do not get confused by this common mistake.
        if strings::has_prefix(Arc::new(Mutex::new(Some((*p).clone()))), Arc::new(Mutex::new(Some("~".to_string())))) {
                // Path segments starting with ~ on Unix are almost always
                // users who have incorrectly quoted ~ while setting GOPATH,
                // preventing it from expanding to $HOME.
                // The situation is made more confusing by the fact that
                // bash allows quoted ~ in $PATH (most shells do not).
                // Do not get confused by this, and do not try to use the path.
                // It does not exist, and printing errors about it confuses
                // those users even more, because they think "sure ~ exists!".
                // The go command diagnoses this situation and prints a
                // useful error.
                // On Windows, ~ is used in short names, such as c:\progra~1
                // for c:\program files.
        continue
    }
                // Path segments starting with ~ on Unix are almost always
                // users who have incorrectly quoted ~ while setting GOPATH,
                // preventing it from expanding to $HOME.
                // The situation is made more confusing by the fact that
                // bash allows quoted ~ in $PATH (most shells do not).
                // Do not get confused by this, and do not try to use the path.
                // It does not exist, and printing errors about it confuses
                // those users even more, because they think "sure ~ exists!".
                // The go command diagnoses this situation and prints a
                // useful error.
                // On Windows, ~ is used in short names, such as c:\progra~1
                // for c:\program files.
        { let new_val = { let __append_target = all.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*p).clone()); __append_target.clone() }; all = new_val; };
    } }
                // Empty paths are uninteresting.
                // If the path is the GOROOT, ignore it.
                // People sometimes set GOPATH=$GOROOT.
                // Do not get confused by this common mistake.
                // Path segments starting with ~ on Unix are almost always
                // users who have incorrectly quoted ~ while setting GOPATH,
                // preventing it from expanding to $HOME.
                // The situation is made more confusing by the fact that
                // bash allows quoted ~ in $PATH (most shells do not).
                // Do not get confused by this, and do not try to use the path.
                // It does not exist, and printing errors about it confuses
                // those users even more, because they think "sure ~ exists!".
                // The go command diagnoses this situation and prints a
                // useful error.
                // On Windows, ~ is used in short names, such as c:\progra~1
                // for c:\program files.
        return all.clone();
    }

    /// SrcDirs returns a list of package source root directories.
    /// It draws from the current Go root and Go path but omits directories
    /// that do not exist.
    pub fn src_dirs(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut all: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x != __tmp_y } {
        let mut dir = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string()])))) };
        if self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = { let __append_target = all.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*dir.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; all = new_val; };
    }
    }
        { let __range_holder = self.gopath().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let mut dir = self.join_path(Arc::new(Mutex::new(Some(vec![(*p).clone(), "src".to_string()]))));
        if self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = { let __append_target = all.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*dir.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; all = new_val; };
    }
    } }
        return all.clone();
    }

    /// ImportDir is like [Import] but processes the Go package found in
    /// the named directory.
    pub fn import_dir(&self, dir: Arc<Mutex<Option<String>>>, mode: Arc<Mutex<Option<ImportMode>>>) -> (Arc<Mutex<Option<Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        self.import(Arc::new(Mutex::new(Some(".".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Import returns details about the Go package named by the import path,
    /// interpreting local import paths relative to the srcDir directory.
    /// If the path is a local import path naming a package that can be imported
    /// using a standard import path, the returned package will set p.ImportPath
    /// to that path.
    ///
    /// In the directory containing the package, .go, .c, .h, and .s files are
    /// considered part of the package except for:
    ///
    ///   - .go files in package documentation
    ///   - files starting with _ or . (likely editor temporary files)
    ///   - files with build constraints not satisfied by the context
    ///
    /// If an error occurs, Import returns a non-nil error and a non-nil
    /// *[Package] containing partial information.
    pub fn import(&self, path: Arc<Mutex<Option<String>>>, srcDir: Arc<Mutex<Option<String>>>, mode: Arc<Mutex<Option<ImportMode>>>) -> (Arc<Mutex<Option<Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut p = Arc::new(Mutex::new(Some(Package { import_path: Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (p.clone(), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("import {:?}: invalid import path", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }

        let mut pkgtargetroot: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut pkga: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut pkgerr: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut suffix = Arc::new(Mutex::new(Some("".to_string())));
        if { let __tmp_x = (*self.install_suffix.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = format!("{}{}", "_".to_string(), (*self.install_suffix.clone().lock().unwrap().as_ref().unwrap())); *suffix.lock().unwrap() = Some(new_val); };
    }
        { let _switch_val = { let __selector_holder = self.compiler.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == ("gccgo".to_string()) {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "pkg/gccgo_".to_string())); __s.push_str(&format!("{}", (*self.g_o_o_s.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", "_".to_string())); __s.push_str(&format!("{}", (*self.g_o_a_r_c_h.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", { let __v = (*suffix.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s }; *pkgtargetroot.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("gc".to_string()) {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "pkg/".to_string())); __s.push_str(&format!("{}", (*self.g_o_o_s.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", "_".to_string())); __s.push_str(&format!("{}", (*self.g_o_a_r_c_h.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", { let __v = (*suffix.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s }; *pkgtargetroot.lock().unwrap() = Some(new_val); };
        } else {
                        // Save error for end of function.
            { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("import {:?}: unknown compiler {:?}", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*self.compiler.lock().unwrap().as_ref().unwrap())))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *pkgerr.lock().unwrap() = new_val; };
        }
    }
                // Save error for end of function.
        let mut ctxt_closure_clone = (*self).clone(); let p_closure_clone = p.clone(); let mut pkga_closure_clone = pkga.clone(); let pkgtargetroot_closure_clone = pkgtargetroot.clone(); let mut setPkga = Arc::new(Mutex::new(Some(Box::new(move || {
        { let _switch_val = { let __selector_holder = ctxt_closure_clone.compiler.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == ("gccgo".to_string()) {
            let (mut dir, mut elem) = pathpkg::split({ let __selector_holder = (*p_closure_clone.lock().unwrap().as_ref().unwrap()).import_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", { let __v = (*pkgtargetroot_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "/".to_string())); __s.push_str(&format!("{}", { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "lib".to_string())); __s.push_str(&format!("{}", { let __v = (*elem.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", ".a".to_string())); __s }; *pkga_closure_clone.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("gc".to_string()) {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", { let __v = (*pkgtargetroot_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "/".to_string())); __s.push_str(&format!("{}", (*{ let __field = (*p_closure_clone.lock().unwrap().as_ref().unwrap()).import_path.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())); __s.push_str(&format!("{}", ".a".to_string())); __s }; *pkga_closure_clone.lock().unwrap() = Some(new_val); };
        }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>)));
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = setPkga.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };

        let mut binaryOnly = Arc::new(Mutex::new(Some(false)));
        'found: {
            if is_local_import(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = "".to_string(); *pkga.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*srcDir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (p.clone(), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("import {:?}: import relative to unknown directory", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }
        if !self.is_abs_path(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = self.join_path(Arc::new(Mutex::new(Some(vec![(*srcDir.lock().unwrap().as_ref().unwrap()).clone(), (*path.lock().unwrap().as_ref().unwrap()).clone()])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = __moved_val; };
    }
                // p.Dir directory may or may not exist. Gather partial information first, check if it exists later.
                // Determine canonical import path, if any.
                // Exclude results where the import path would include /testdata/.
        let mut inTestdata = Arc::new(Mutex::new(Some(Box::new(move |sub: Arc<Mutex<Option<String>>>| -> bool {
        strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/testdata/".to_string())))) || strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/testdata".to_string())))) || strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("testdata/".to_string())))) || { let __tmp_x = (*sub.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "testdata".to_string(); __tmp_x == __tmp_y }
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>)));
        if { let __tmp_x = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let mut root = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string()])))) };
        {
        let (mut sub, mut ok) = self.has_subdir(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if ok && !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = { let mut __f_guard = inTestdata.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) } {
            { let new_val = true; *(*p.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };;
            { let new_val = sub.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).import_path.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*p.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };;
            { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = setPkga.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };;
            break 'found;;
        }
    }
    }
                // p.ImportPath changed
        let mut all = self.gopath();
        { let __range_holder = all.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, root) in __range_values.iter().enumerate() {
        let mut rootsrc = self.join_path(Arc::new(Mutex::new(Some(vec![(*root).clone(), "src".to_string()]))));
        {
        let (mut sub, mut ok) = self.has_subdir(Arc::new(Mutex::new(Some({ let __arg_holder = rootsrc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if ok && !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> = { let mut __f_guard = inTestdata.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) } {
            if { let __tmp_x = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x != __tmp_y } {
        {
        let mut dir = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string(), (*sub.lock().unwrap().as_ref().unwrap()).clone()])))) };;
        if self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).conflict_dir.lock().unwrap() = Some(new_val); };;
            break 'found;;
        }
    }
    };
            for earlyRoot in &{ let __seq_holder = all.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v } {
        {
        let mut dir = self.join_path(Arc::new(Mutex::new(Some(vec![(*earlyRoot).clone(), "src".to_string(), (*sub.lock().unwrap().as_ref().unwrap()).clone()]))));;
        if self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).conflict_dir.lock().unwrap() = Some(new_val); };;
            break 'found;;
        }
    }
    };
            { let new_val = sub.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).import_path.lock().unwrap() = Some(new_val); };;
            { let new_val = (*root).clone(); *(*p.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };;
            { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = setPkga.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };;
            break 'found;;
        }
    }
    } }
    } else {
        if strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/".to_string())))) {
        return (p.clone(), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("import {:?}: cannot import absolute path", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }))))));
    }
        {
        let mut err = self.import_go(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = srcDir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            break 'found;;
        } else if { let __left = err.clone(); let __right = errNoModules.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; !__eq } {
        return (p.clone(), err.clone());
    }
    }
        let mut gopath = self.gopath();
                // tried records the location of unsuccessful package lookups
        let mut tried: Arc<Mutex<Option<AnonymousStruct1>>> = Arc::new(Mutex::new(Some(Default::default())));
                // Vendor directories get first chance to satisfy import.
        if { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & IGNORE_VENDOR as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } && { let __tmp_x = (*srcDir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let mut ctxt_closure_clone = (*self).clone(); let p_closure_clone = p.clone(); let path_closure_clone = path.clone(); let setPkga_closure_clone = setPkga.clone(); let srcDir_closure_clone = srcDir.clone(); let tried_closure_clone = tried.clone(); let mut searchVendor = Arc::new(Mutex::new(Some(Box::new(move |root: Arc<Mutex<Option<String>>>, isGoroot: Arc<Mutex<Option<bool>>>| -> bool {
        let (mut sub, mut ok) = ctxt_closure_clone.has_subdir(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = srcDir_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if !ok || !strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("src/".to_string())))) || strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/testdata/".to_string())))) {
        return false;
    }
        loop {
        let mut vendor = ctxt_closure_clone.join_path(Arc::new(Mutex::new(Some(vec![(*root.lock().unwrap().as_ref().unwrap()).clone(), (*sub.lock().unwrap().as_ref().unwrap()).clone(), "vendor".to_string()]))));
        if ctxt_closure_clone.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = vendor.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        let mut dir = ctxt_closure_clone.join_path(Arc::new(Mutex::new(Some(vec![(*vendor.lock().unwrap().as_ref().unwrap()).clone(), (*path_closure_clone.lock().unwrap().as_ref().unwrap()).clone()]))));
        if ctxt_closure_clone.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && has_go_files(Arc::new(Mutex::new(Some(ctxt_closure_clone.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*p_closure_clone.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };
        { let new_val = strings::trim_prefix(pathpkg::join((sub.clone(), "vendor".to_string(), path_closure_clone.clone())), Arc::new(Mutex::new(Some("src/".to_string())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p_closure_clone.lock().unwrap().as_ref().unwrap()).import_path.lock().unwrap() = __moved_val; };
        { let new_val = isGoroot.lock().unwrap().as_ref().unwrap().clone(); *(*p_closure_clone.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };
        { let new_val = root.lock().unwrap().as_ref().unwrap().clone(); *(*p_closure_clone.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = setPkga_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        return true;
    }
        { let new_val = { let __append_target = (*tried_closure_clone.lock().unwrap().as_ref().unwrap()).vendor.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*dir.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; (*tried_closure_clone.lock().unwrap().as_mut().unwrap()).vendor = new_val; };
    }
        let mut i = strings::last_index(Arc::new(Mutex::new(Some({ let __arg_holder = sub.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/".to_string()))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*sub.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *sub.lock().unwrap() = __moved_val; };
    }
        false
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<bool>>>) -> bool + Send + Sync>)));
                // p.ImportPath changed
        if { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<bool>>>) -> bool + Send + Sync> = { let mut __f_guard = searchVendor.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<bool>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(true)))) } {
        break 'found;
    }
        { let __range_holder = gopath.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for root in __range_values.iter() {
        if { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<bool>>>) -> bool + Send + Sync> = { let mut __f_guard = searchVendor.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<bool>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(root.clone()))), Arc::new(Mutex::new(Some(false)))) } {
        break 'found;
    }
    } }
    }
                // p.ImportPath changed
                // Determine directory from import path.
        if { let __tmp_x = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // If the package path starts with "vendor/", only search GOROOT before
                // GOPATH if the importer is also within GOROOT. That way, if the user has
                // vendored in a package that is subsequently included in the standard
                // distribution, they'll continue to pick up their own vendored copy.
        let mut gorootFirst = Arc::new(Mutex::new(Some({ let __tmp_x = (*srcDir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } || !strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("vendor/".to_string())))))));
        if !{ let __v = (*gorootFirst.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let (__tmp_0, __tmp_1) = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __arg_holder = srcDir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.has_subdir(__method_arg0, __method_arg1) }; *gorootFirst.lock().unwrap() = Some(__tmp_1); };
    }
        if { let __v = (*gorootFirst.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut dir = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string(), (*path.lock().unwrap().as_ref().unwrap()).clone()])))) };
        if { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x != __tmp_y } {
        let mut isDir = self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = !isDir && { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ALLOW_BINARY as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } && { let __tmp_x = (*pkga.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __method_arg0 = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*pkga.lock().unwrap().as_ref().unwrap()).clone()])))) }; self.is_file(__method_arg0) }; *binaryOnly.lock().unwrap() = Some(new_val); };
        if isDir || { let __v = (*binaryOnly.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };
        { let new_val = true; *(*p.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*p.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };
        break 'found;
    }
    }
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*tried.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x == __tmp_y } && goroot::is_standard_package({ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __selector_holder = self.compiler.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) {
                // TODO(bcmills): Setting p.Dir here is misleading, because gccgo
                // doesn't actually load its standard-library packages from this
                // directory. See if we can leave it unset.
        { let new_val = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string(), (*path.lock().unwrap().as_ref().unwrap()).clone()])))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = __moved_val; };
        { let new_val = true; *(*p.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*p.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };
        break 'found;
    }
    }
                // If the package path starts with "vendor/", only search GOROOT before
                // GOPATH if the importer is also within GOROOT. That way, if the user has
                // vendored in a package that is subsequently included in the standard
                // distribution, they'll continue to pick up their own vendored copy.
                // TODO(bcmills): Setting p.Dir here is misleading, because gccgo
                // doesn't actually load its standard-library packages from this
                // directory. See if we can leave it unset.
        { let __range_holder = gopath.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for root in __range_values.iter() {
        let mut dir = self.join_path(Arc::new(Mutex::new(Some(vec![(*root).clone(), "src".to_string(), (*path.lock().unwrap().as_ref().unwrap()).clone()]))));
        let mut isDir = self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = !isDir && { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ALLOW_BINARY as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } && { let __tmp_x = (*pkga.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __method_arg0 = self.join_path(Arc::new(Mutex::new(Some(vec![(*root).clone(), (*pkga.lock().unwrap().as_ref().unwrap()).clone()])))); self.is_file(__method_arg0) }; *binaryOnly.lock().unwrap() = Some(new_val); };
        if isDir || { let __v = (*binaryOnly.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };
        { let new_val = (*root).clone(); *(*p.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };
        break 'found;
    }
        { let new_val = { let __append_target = (*tried.lock().unwrap().as_ref().unwrap()).gopath.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*dir.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; (*tried.lock().unwrap().as_mut().unwrap()).gopath = new_val; };
    } }
                // If we tried GOPATH first due to a "vendor/" prefix, fall back to GOPATH.
                // That way, the user can still get useful results from 'go list' for
                // standard-vendored paths passed on the command line.
        if { let __tmp_x = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = { let __selector_holder = (*tried.lock().unwrap().as_ref().unwrap()).goroot.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        let mut dir = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string(), (*path.lock().unwrap().as_ref().unwrap()).clone()])))) };
        if { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x != __tmp_y } {
        let mut isDir = self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = !isDir && { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ALLOW_BINARY as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } && { let __tmp_x = (*pkga.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __method_arg0 = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*pkga.lock().unwrap().as_ref().unwrap()).clone()])))) }; self.is_file(__method_arg0) }; *binaryOnly.lock().unwrap() = Some(new_val); };
        if isDir || { let __v = (*binaryOnly.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };
        { let new_val = true; *(*p.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*p.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };
        break 'found;
    }
    }
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*tried.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };
    }
                // package was not found
        let mut paths: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        let mut format = Arc::new(Mutex::new(Some("\t%s (vendor tree)".to_string())));
        { let __range_holder = (*tried.lock().unwrap().as_ref().unwrap()).vendor.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for dir in __range_values.iter() {
        { let new_val = { let __append_target = paths.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("{}", (*format.lock().unwrap().as_ref().unwrap()).clone())))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; paths = new_val; };
        { let new_val = "\t%s".to_string(); *format.lock().unwrap() = Some(new_val); };
    } }
        if { let __tmp_x = { let __selector_holder = (*tried.lock().unwrap().as_ref().unwrap()).goroot.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = paths.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("\t{} (from $GOROOT)", (*{ let __field = (*tried.lock().unwrap().as_ref().unwrap()).goroot.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; paths = new_val; };
    } else {
        { let new_val = { let __append_target = paths.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push("\t($GOROOT not set)".to_string()); __append_target.clone() }; paths = new_val; };
    }
        { let new_val = "\t%s (from $GOPATH)".to_string(); *format.lock().unwrap() = Some(new_val); };
        { let __range_holder = (*tried.lock().unwrap().as_ref().unwrap()).gopath.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for dir in __range_values.iter() {
        { let new_val = { let __append_target = paths.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(format!("{}", (*format.lock().unwrap().as_ref().unwrap()).clone())))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; paths = new_val; };
        { let new_val = "\t%s".to_string(); *format.lock().unwrap() = Some(new_val); };
    } }
        if { let __tmp_x = (({ let __len_target = { let __field = (*tried.lock().unwrap().as_ref().unwrap()).gopath.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = paths.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push("\t($GOPATH not set. For more details see: 'go help gopath')".to_string()); __append_target.clone() }; paths = new_val; };
    }
        return (p.clone(), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("cannot find package {:?} in any of:\n{}", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*strings::join(paths.clone(), Arc::new(Mutex::new(Some("\n".to_string())))).lock().unwrap().as_ref().unwrap())))))));
    }

        }
                // local imports have no installed path
                // p.Dir directory may or may not exist. Gather partial information first, check if it exists later.
                // Determine canonical import path, if any.
                // Exclude results where the import path would include /testdata/.
                // p.ImportPath changed
                // We found a potential import path for dir,
                // but check that using it wouldn't find something
                // else first.
                // sub would not name some other directory instead of this one.
                // Record it.
                // p.ImportPath changed
                // It's okay that we didn't find a root containing dir.
                // Keep going with the information we have.
                // needed twice below; avoid computing many times
                // tried records the location of unsuccessful package lookups
                // Vendor directories get first chance to satisfy import.
                // p.ImportPath changed
                // Determine directory from import path.
                // If the package path starts with "vendor/", only search GOROOT before
                // GOPATH if the importer is also within GOROOT. That way, if the user has
                // vendored in a package that is subsequently included in the standard
                // distribution, they'll continue to pick up their own vendored copy.
                // TODO(bcmills): Setting p.Dir here is misleading, because gccgo
                // doesn't actually load its standard-library packages from this
                // directory. See if we can leave it unset.
                // If we tried GOPATH first due to a "vendor/" prefix, fall back to GOPATH.
                // That way, the user can still get useful results from 'go list' for
                // standard-vendored paths passed on the command line.
                // package was not found
        if { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).root.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).root.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string()])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).src_root.lock().unwrap() = __moved_val; };
        { let new_val = self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).root.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "pkg".to_string()])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).pkg_root.lock().unwrap() = __moved_val; };
        { let new_val = self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).root.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "bin".to_string()])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).bin_dir.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*pkga.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // Always set PkgTargetRoot. It might be used when building in shared
                // mode.
        { let new_val = self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).root.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*pkgtargetroot.lock().unwrap().as_ref().unwrap()).clone()])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).pkg_target_root.lock().unwrap() = __moved_val; };
                // Set the install target if applicable.
        if !(*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).goroot.clone(); __field }.lock().unwrap().as_ref().unwrap()) || ({ let __tmp_x = (*{ let __recv_holder = (*installgoroot.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).value(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "all".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).import_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "unsafe".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).import_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "builtin".to_string(); __tmp_x != __tmp_y }) {
        if (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).goroot.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __recv_holder = (*installgoroot.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.inc_non_default(); __result };
    }
        { let new_val = self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).root.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*pkga.lock().unwrap().as_ref().unwrap()).clone()])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).pkg_obj.lock().unwrap() = __moved_val; };
    }
    }
    }

                // Always set PkgTargetRoot. It might be used when building in shared
                // mode.
                // Set the install target if applicable.
                // If it's a local import path, by the time we get here, we still haven't checked
                // that p.Dir directory exists. This is the right time to do that check.
                // We can't do it earlier, because we want to gather partial information for the
                // non-nil *Package returned when an error occurs.
                // We need to do this before we return early on FindOnly flag.
        if is_local_import(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !self.is_dir(Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        if { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x == __tmp_y } && (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).goroot.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // gccgo has no sources for GOROOT packages.
        return (p.clone(), Arc::new(Mutex::new(None)));
    }
                // gccgo has no sources for GOROOT packages.
                // package was not found
        return (p.clone(), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("cannot find package {:?} in:\n\t{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).import_path.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone(), (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()))))));
    }

                // gccgo has no sources for GOROOT packages.
                // package was not found
        if { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & FIND_ONLY as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        return (p.clone(), pkgerr.clone());
    }
        if { let __v = (*binaryOnly.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ALLOW_BINARY as u64)))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        return (p.clone(), pkgerr.clone());
    }

        if { let __tmp_x = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gccgo".to_string(); __tmp_x == __tmp_y } && (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).goroot.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // gccgo has no sources for GOROOT packages.
        return (p.clone(), Arc::new(Mutex::new(None)));
    }

                // gccgo has no sources for GOROOT packages.
        let (mut dirs, mut err) = self.read_dir(Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (p.clone(), err.clone());
    }

        let mut badGoError: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut badGoFiles = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::new())));
        let mut badGoError_closure_clone = badGoError.clone(); let badGoFiles_closure_clone = badGoFiles.clone(); let p_closure_clone = p.clone(); let mut badGoFile = Arc::new(Mutex::new(Some(Box::new(move |name: Arc<Mutex<Option<String>>>, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>| {
        if { let __nil_result = (*badGoError_closure_clone.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *badGoError_closure_clone.lock().unwrap() = new_val; };
    }
        if !{ let __map = { let __map_holder = badGoFiles_closure_clone.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        { let new_val = { let __append_target = (*p_closure_clone.lock().unwrap().as_ref().unwrap()).invalid_go_files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; (*p_closure_clone.lock().unwrap().as_mut().unwrap()).invalid_go_files = new_val; };
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(true))); (*badGoFiles_closure_clone.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync>)));

        let mut Sfiles: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        let mut firstFile: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut firstCommentFile: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut embedPos = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<token_Position>>>>>::new())));
        let mut testEmbedPos = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<token_Position>>>>>::new())));
        let mut xTestEmbedPos = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<token_Position>>>>>::new())));
        let mut importPos = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<token_Position>>>>>::new())));
        let mut testImportPos = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<token_Position>>>>>::new())));
        let mut xTestImportPos = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<token_Position>>>>>::new())));
        let mut allTags = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::new())));
        let mut fset = token::new_file_set();
        { let __range_holder = dirs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for d in __range_values.iter() {
        if d.is_dir() {
        continue
    }
        if { let __tmp_x = (*d.r#type().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = fs::MODE_SYMLINK; __tmp_x == __tmp_y } {
        if { let __method_arg0 = self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*d.name().lock().unwrap().as_ref().unwrap()).clone()])))); self.is_dir(__method_arg0) } {
                // Symlinks to directories are not source files.
        continue
    }
    }
                // Symlinks to directories are not source files.
        let mut name = d.name();
        let mut ext = name_ext(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let (mut info, mut err) = self.match_file_1(Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), allTags.clone(), (*p.lock().unwrap().as_ref().unwrap()).binary_only.clone(), fset.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } && strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".go".to_string())))) {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = badGoFile.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err.clone()) };
        continue
    }
        if { let __nil_result = (*info.lock().unwrap()).is_none(); __nil_result } {
        if strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_".to_string())))) || strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".".to_string())))) {
    } else if { let __tmp_x = (*ext.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".go".to_string(); __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = (*p.lock().unwrap().as_ref().unwrap()).ignored_go_files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; (*p.lock().unwrap().as_mut().unwrap()).ignored_go_files = new_val; };
    } else if { let __nil_result = (*file_list_for_ext(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = ext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __append_target = (*p.lock().unwrap().as_ref().unwrap()).ignored_other_files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; (*p.lock().unwrap().as_mut().unwrap()).ignored_other_files = new_val; };
    }
                // not due to build constraints - don't report
        continue
    }
                // not due to build constraints - don't report
                // Going to save the file. For non-Go files, can stop here.
        { let _switch_val = (*ext.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (".go".to_string()) {
        } else if _switch_val == (".S".to_string()) || _switch_val == (".sx".to_string()) {
                        // special case for cgo, handled at end
            { let new_val = { let __append_target = Sfiles.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; Sfiles = new_val; };
            continue
        } else {
            {
        let mut list = file_list_for_ext(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = ext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*list.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; let __cloned_val = { let __guard = new_val.lock().unwrap(); (*__guard).clone() }; *list.lock().unwrap() = __cloned_val; };;
        }
    }
            continue
        }
    }
                // keep going
                // special case for cgo, handled at end
        let (mut data, mut filename) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*info.lock().unwrap().as_ref().unwrap()).header.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*info.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __nil_target = (*info.lock().unwrap().as_ref().unwrap()).parse_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = badGoFile.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*info.lock().unwrap().as_ref().unwrap()).parse_err.clone(); __field }) };
    }
                // Fall through: we might still have a partial AST in info.parsed,
                // and we want to list files with parse errors anyway.
        let mut pkg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __nil_target = (*info.lock().unwrap().as_ref().unwrap()).parsed.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __selector_holder = (*(*(*info.lock().unwrap().as_ref().unwrap()).parsed.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *pkg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*pkg.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "documentation".to_string(); __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = (*p.lock().unwrap().as_ref().unwrap()).ignored_go_files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; (*p.lock().unwrap().as_mut().unwrap()).ignored_go_files = new_val; };
        continue
    }
    }
        let mut isTest = strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_test.go".to_string()))));
        let mut isXTest = Arc::new(Mutex::new(Some(false)));
        if isTest && strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = pkg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_test".to_string())))) && { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*pkg.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        { let new_val = true; *isXTest.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*pkg.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = ((*pkg.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 5; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pkg.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = pkg.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
        { let new_val = name.lock().unwrap().as_ref().unwrap().clone(); *firstFile.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*pkg.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x != __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = badGoFile.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(MultiplePackageError { dir: Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), packages: Arc::new(Mutex::new(Some(vec![{ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*pkg.lock().unwrap().as_ref().unwrap()).clone()]))), files: Arc::new(Mutex::new(Some(vec![(*firstFile.lock().unwrap().as_ref().unwrap()).clone(), (*name.lock().unwrap().as_ref().unwrap()).clone()]))), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))) };
    }
                // TODO(#45999): The choice of p.Name is arbitrary based on file iteration
                // order. Instead of resolving p.Name arbitrarily, we should clear out the
                // existing name and mark the existing files as also invalid.
                // Grab the first package comment as docs, provided it is not from a test file.
        if { let __nil_target = (*info.lock().unwrap().as_ref().unwrap()).parsed.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __nil_target = (*(*info.lock().unwrap().as_ref().unwrap()).parsed.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).doc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } && !isTest && !{ let __v = (*isXTest.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = doc::synopsis((*(*(*info.lock().unwrap().as_ref().unwrap()).parsed.lock().unwrap().as_ref().unwrap()).doc.lock().unwrap().as_mut().unwrap()).text()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).doc.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & IMPORT_COMMENT as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        let (mut qcom, mut line) = find_import_comment(data.clone());
        if { let __tmp_x = line; let __tmp_y = 0; __tmp_x != __tmp_y } {
        let (mut com, mut err) = strconv::unquote({ let __arg_holder = qcom.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = badGoFile.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}:{}: cannot parse import comment", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }, line)))))) };
    } else if { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).import_comment.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = com.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).import_comment.lock().unwrap() = Some(new_val); };
        { let new_val = name.lock().unwrap().as_ref().unwrap().clone(); *firstCommentFile.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).import_comment.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*com.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = badGoFile.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("found import comments {:?} ({}) and {:?} ({}) in {}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).import_comment.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone(), { let __v = (*firstCommentFile.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*com.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())))))) };
    }
    }
    }
                // Record imports and information about cgo.
        let mut isCgo = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = (*info.lock().unwrap().as_ref().unwrap()).imports.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for imp in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = imp.path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "C".to_string(); __tmp_x == __tmp_y } {
        if isTest {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = badGoFile.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("use of cgo in test {} not supported", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v })))))) };
        continue
    }
        { let new_val = true; *isCgo.lock().unwrap() = Some(new_val); };
        if { let __nil_target = imp.doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        {
        let mut err = self.save_cgo(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone(), { let __field = imp.doc.clone(); __field });;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = badGoFile.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err.clone()) };;
        }
    }
    }
    }
    } }
        let mut fileList: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        let mut importMap: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));let mut embedMap: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));
        let mut directives: Arc<Mutex<Option<Vec<Directive>>>> = Arc::new(Mutex::new(None));
        if { let __v = (*isCgo.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            { let __map_key = "cgo".to_string(); let __map_value = Arc::new(Mutex::new(Some(true))); (*allTags.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
            if (*self.cgo_enabled.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).cgo_files.clone().clone(); fileList = new_val; };
        { let new_val = importPos.clone(); importMap = new_val; };
        { let new_val = embedPos.clone(); embedMap = new_val; };
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).directives.clone().clone(); directives = new_val; };
    } else {
                // Ignore imports and embeds from cgo files if cgo is disabled.
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).ignored_go_files.clone().clone(); fileList = new_val; };
    }
        } else if { let __v = (*isXTest.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            { let new_val = (*p.lock().unwrap().as_ref().unwrap()).x_test_go_files.clone().clone(); fileList = new_val; };
            { let new_val = xTestImportPos.clone(); importMap = new_val; };
            { let new_val = xTestEmbedPos.clone(); embedMap = new_val; };
            { let new_val = (*p.lock().unwrap().as_ref().unwrap()).x_test_directives.clone().clone(); directives = new_val; };
        } else if isTest {
            { let new_val = (*p.lock().unwrap().as_ref().unwrap()).test_go_files.clone().clone(); fileList = new_val; };
            { let new_val = testImportPos.clone(); importMap = new_val; };
            { let new_val = testEmbedPos.clone(); embedMap = new_val; };
            { let new_val = (*p.lock().unwrap().as_ref().unwrap()).test_directives.clone().clone(); directives = new_val; };
        } else {
            { let new_val = (*p.lock().unwrap().as_ref().unwrap()).go_files.clone().clone(); fileList = new_val; };
            { let new_val = importPos.clone(); importMap = new_val; };
            { let new_val = embedPos.clone(); embedMap = new_val; };
            { let new_val = (*p.lock().unwrap().as_ref().unwrap()).directives.clone().clone(); directives = new_val; };
        }
                // Ignore imports and embeds from cgo files if cgo is disabled.
        { let new_val = { let __append_target = fileList.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; let __cloned_val = { let __guard = new_val.lock().unwrap(); (*__guard).clone() }; *fileList.lock().unwrap() = __cloned_val; };
        if { let __nil_result = (*importMap.lock().unwrap()).is_some(); __nil_result } {
        { let __range_holder = (*info.lock().unwrap().as_ref().unwrap()).imports.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for imp in __range_values.iter() {
        { let __map_key = { let __selector_holder = imp.path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __map_value = { let __slice = { let __map_holder = importMap.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&{ let __selector_holder = imp.path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push((*{ let __recv = fset.clone(); let __recv_ptr: *mut token_FileSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut token_FileSet }; let __result = unsafe { &mut *__recv_ptr }.position({ let __selector_holder = imp.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }); __result }.lock().unwrap().as_ref().unwrap()).clone()); __slice.clone() }; (*importMap.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    }
        if { let __nil_result = (*embedMap.lock().unwrap()).is_some(); __nil_result } {
        { let __range_holder = (*info.lock().unwrap().as_ref().unwrap()).embeds.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for emb in __range_values.iter() {
        { let __map_key = { let __selector_holder = emb.pattern.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __map_value = { let __slice = { let __map_holder = embedMap.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&{ let __selector_holder = emb.pattern.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __selector_holder = emb.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }); __slice.clone() }; (*embedMap.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    }
        if { let __nil_result = (*directives.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __append_target = directives.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = (*info.lock().unwrap().as_ref().unwrap()).directives.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; let __cloned_val = { let __guard = new_val.lock().unwrap(); (*__guard).clone() }; *directives.lock().unwrap() = __cloned_val; };
    }
    } }

                // Symlinks to directories are not source files.
                // not due to build constraints - don't report
                // Going to save the file. For non-Go files, can stop here.
                // keep going
                // special case for cgo, handled at end
                // Fall through: we might still have a partial AST in info.parsed,
                // and we want to list files with parse errors anyway.
                // TODO(#45999): The choice of p.Name is arbitrary based on file iteration
                // order. Instead of resolving p.Name arbitrarily, we should clear out the
                // existing name and mark the existing files as also invalid.
                // Grab the first package comment as docs, provided it is not from a test file.
                // Record imports and information about cgo.
                // Ignore imports and embeds from cgo files if cgo is disabled.
        for (tag, _) in { let __range_holder = allTags.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = (*p.lock().unwrap().as_ref().unwrap()).all_tags.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(tag.clone()); __append_target.clone() }; (*p.lock().unwrap().as_mut().unwrap()).all_tags = new_val; };
    }
        { let __sort_target = (*p.lock().unwrap().as_ref().unwrap()).all_tags.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };

        { let (__tmp_0, __tmp_1) = clean_decls(embedPos.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).embed_patterns.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).embed_pattern_pos.lock().unwrap() = __moved_tmp_1; };
        { let (__tmp_0, __tmp_1) = clean_decls(testEmbedPos.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).test_embed_patterns.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).test_embed_pattern_pos.lock().unwrap() = __moved_tmp_1; };
        { let (__tmp_0, __tmp_1) = clean_decls(xTestEmbedPos.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).x_test_embed_patterns.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).x_test_embed_pattern_pos.lock().unwrap() = __moved_tmp_1; };

        { let (__tmp_0, __tmp_1) = clean_decls(importPos.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).imports.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).import_pos.lock().unwrap() = __moved_tmp_1; };
        { let (__tmp_0, __tmp_1) = clean_decls(testImportPos.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).test_imports.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).test_import_pos.lock().unwrap() = __moved_tmp_1; };
        { let (__tmp_0, __tmp_1) = clean_decls(xTestImportPos.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).x_test_imports.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*p.lock().unwrap().as_ref().unwrap()).x_test_import_pos.lock().unwrap() = __moved_tmp_1; };

                // add the .S/.sx files only if we are using cgo
                // (which means gcc will compile them).
                // The standard assemblers expect .s files.
        if { let __tmp_x = (({ let __len_target = { let __field = (*p.lock().unwrap().as_ref().unwrap()).cgo_files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = (*p.lock().unwrap().as_ref().unwrap()).s_files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Sfiles.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*p.lock().unwrap().as_mut().unwrap()).s_files = new_val; };
        { let __sort_target = (*p.lock().unwrap().as_ref().unwrap()).s_files.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    } else {
        { let new_val = { let __append_target = (*p.lock().unwrap().as_ref().unwrap()).ignored_other_files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Sfiles.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*p.lock().unwrap().as_mut().unwrap()).ignored_other_files = new_val; };
        { let __sort_target = (*p.lock().unwrap().as_ref().unwrap()).ignored_other_files.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    }

        if { let __nil_result = (*badGoError.lock().unwrap()).is_some(); __nil_result } {
        return (p.clone(), badGoError.clone());
    }
        if { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (({ let __len_target = { let __field = (*p.lock().unwrap().as_ref().unwrap()).go_files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __len_target = { let __field = (*p.lock().unwrap().as_ref().unwrap()).cgo_files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = (*p.lock().unwrap().as_ref().unwrap()).test_go_files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = (*p.lock().unwrap().as_ref().unwrap()).x_test_go_files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (p.clone(), Arc::new(Mutex::new(Some(Box::new(NoGoError { dir: Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
        return (p.clone(), pkgerr.clone());
        unreachable!()
    }

    /// importGo checks whether it can use the go command to find the directory for path.
    /// If using the go command is not appropriate, importGo returns errNoModules.
    /// Otherwise, importGo tries using the go command and reports whether that succeeded.
    /// Using the go command lets build.Import and build.Context.Import find code
    /// in Go modules. In the long term we want tools to use go/packages (currently golang.org/x/tools/go/packages),
    /// which will also use the go command.
    /// Invoking the go command here is not very efficient in that it computes information
    /// about the requested package and all dependencies and then only reports about the requested package.
    /// Then we reinvoke it for every dependency. But this is still better than not working at all.
    /// See golang.org/issue/26504.
    pub fn import_go(&self, p: Arc<Mutex<Option<Package>>>, path: Arc<Mutex<Option<String>>>, srcDir: Arc<Mutex<Option<String>>>, mode: Arc<Mutex<Option<ImportMode>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
                // To invoke the go command,
                // we must not being doing special things like AllowBinary or IgnoreVendor,
                // and all the file system callbacks must be nil (we're meant to use the local file system).
        if { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ALLOW_BINARY as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } || { let __tmp_x = ImportMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & IGNORE_VENDOR as u64))))); let __tmp_y = ImportMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } || { let __nil_target = self.join_path.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = self.split_path_list.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = self.is_abs_path.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = self.is_dir.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = self.has_subdir.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = self.read_dir.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = self.open_file.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || !equal({ let __field = self.tool_tags.clone(); __field }, defaultToolTags.clone()) || !equal({ let __field = self.release_tags.clone(); __field }, defaultReleaseTags.clone()) {
        return errNoModules.clone();
    }
                // If ctxt.GOROOT is not set, we don't know which go command to invoke,
                // and even if we did we might return packages in GOROOT that we wouldn't otherwise find
                // (because we don't know to search in 'go env GOROOT' otherwise).
        if { let __tmp_x = (*self.g_o_r_o_o_t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return errNoModules.clone();
    }
                // Predict whether module aware mode is enabled by checking the value of
                // GO111MODULE and looking for a go.mod file in the source directory or
                // one of its parents. Running 'go env GOMOD' in the source directory would
                // give a canonical answer, but we'd prefer not to execute another command.
        let mut go111Module = os::getenv("GO111MODULE".to_string());
        { let _switch_val = (*go111Module.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("off".to_string()) {
            return errNoModules.clone();
        } else {
        }
    }
                // "", "on", "auto", anything else
                // Maybe use modules.
        if { let __tmp_x = (*srcDir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let mut absSrcDir: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if path_filepath::is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = srcDir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = srcDir.lock().unwrap().as_ref().unwrap().clone(); *absSrcDir.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*self.dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("go/build: Dir is non-empty, so relative srcDir is not allowed: {}", { let __v = (*srcDir.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
    } else {
        let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let (__tmp_0, __tmp_1) = path_filepath::abs(Arc::new(Mutex::new(Some({ let __arg_holder = srcDir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *absSrcDir.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return errNoModules.clone();
    }
    }
                // Find the absolute source directory. hasSubdir does not handle
                // relative paths (and can't because the callbacks don't support this).
                // If the source directory is in GOROOT, then the in-process code works fine
                // and we should keep using it. Moreover, the 'go list' approach below doesn't
                // take standard-library vendoring into account and will fail.
        {
        let (_, mut ok) = { let __method_arg0 = path_filepath::join(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string()])))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __arg_holder = absSrcDir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.has_subdir(__method_arg0, __method_arg1) };;
        if ok {
            return errNoModules.clone();;
        }
    }
    }
                // Find the absolute source directory. hasSubdir does not handle
                // relative paths (and can't because the callbacks don't support this).
                // If the source directory is in GOROOT, then the in-process code works fine
                // and we should keep using it. Moreover, the 'go list' approach below doesn't
                // take standard-library vendoring into account and will fail.
                // For efficiency, if path is a standard library package, let the usual lookup code handle it.
        {
        let mut dir = { self.join_path(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "src".to_string(), (*path.lock().unwrap().as_ref().unwrap()).clone()])))) };;
        if self.is_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return errNoModules.clone();;
        }
    }
                // If GO111MODULE=auto, look to see if there is a go.mod.
                // Since go1.13, it doesn't matter if we're inside GOPATH.
        if { let __tmp_x = (*go111Module.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "auto".to_string(); __tmp_x == __tmp_y } {
        let mut parent: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = os::getwd(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *parent.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
                // A nonexistent working directory can't be in a module.
        return errNoModules.clone();
    }
    } else {
        { let (__tmp_0, __tmp_1) = path_filepath::abs(Arc::new(Mutex::new(Some({ let __selector_holder = self.dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *parent.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
                // If the caller passed a bogus Dir explicitly, that's materially
                // different from not having modules enabled.
        return err.clone();
    }
    }
                // A nonexistent working directory can't be in a module.
                // If the caller passed a bogus Dir explicitly, that's materially
                // different from not having modules enabled.
        loop {
        {
        let (mut f, mut err) = { let __method_arg0 = self.join_path(Arc::new(Mutex::new(Some(vec![(*parent.lock().unwrap().as_ref().unwrap()).clone(), "go.mod".to_string()])))); self.open_file(__method_arg0) };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            let mut buf = Arc::new(Mutex::new(Some(vec![0; (100) as usize])));;
            let (_, mut err) = (*f.lock().unwrap().as_ref().unwrap()).read(buf.clone());;
            (*f.lock().unwrap().as_ref().unwrap()).close();;
            if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } || { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        break
    };
        }
    }
                // go.mod exists and is readable (is a file, not a directory).
        let mut d = path_filepath::dir(Arc::new(Mutex::new(Some({ let __arg_holder = parent.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ((*d.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*parent.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } {
        return errNoModules.clone();
    }
                // reached top of file system, no go.mod
        { let new_val = d.lock().unwrap().as_ref().unwrap().clone(); *parent.lock().unwrap() = Some(new_val); };
    }
    }
                // A nonexistent working directory can't be in a module.
                // If the caller passed a bogus Dir explicitly, that's materially
                // different from not having modules enabled.
                // go.mod exists and is readable (is a file, not a directory).
                // reached top of file system, no go.mod
        let mut goCmd = path_filepath::join(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, "bin".to_string(), "go".to_string()]))));
        let mut cmd = exec::command({ let __arg_holder = goCmd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some(vec!["list".to_string(), "-e".to_string(), format!("{}{}", "-compiler=".to_string(), (*self.compiler.clone().lock().unwrap().as_ref().unwrap())), format!("{}{}", "-tags=".to_string(), (*strings::join({ let __field = self.build_tags.clone(); __field }, Arc::new(Mutex::new(Some(",".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}{}", "-installsuffix=".to_string(), (*self.install_suffix.clone().lock().unwrap().as_ref().unwrap())), "-f={{.Dir}}\n{{.ImportPath}}\n{{.Root}}\n{{.Goroot}}\n{{if .Error}}{{.Error}}{{end}}\n".to_string(), "--".to_string(), (*path.lock().unwrap().as_ref().unwrap()).clone()]))));
        if { let __tmp_x = (*self.dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __selector_holder = self.dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*cmd.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };
    }
        let mut stdout: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));let mut stderr: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = io_Writer::__go_from(stdout.clone()); *(*cmd.lock().unwrap().as_ref().unwrap()).stdout.lock().unwrap() = Some(new_val); };
        { let new_val = io_Writer::__go_from(stderr.clone()); *(*cmd.lock().unwrap().as_ref().unwrap()).stderr.lock().unwrap() = Some(new_val); };
        let mut cgo = Arc::new(Mutex::new(Some("0".to_string())));
        if (*self.cgo_enabled.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = "1".to_string(); *cgo.lock().unwrap() = Some(new_val); };
    }
        { let new_val = { let __append_target = { let __recv = cmd.clone(); let __recv_ptr: *mut exec_Cmd = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut exec_Cmd }; let __result = unsafe { &mut *__recv_ptr }.environ(); __result }.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![format!("{}{}", "GOOS=".to_string(), (*self.g_o_o_s.clone().lock().unwrap().as_ref().unwrap())), format!("{}{}", "GOARCH=".to_string(), (*self.g_o_a_r_c_h.clone().lock().unwrap().as_ref().unwrap())), format!("{}{}", "GOROOT=".to_string(), (*self.g_o_r_o_o_t.clone().lock().unwrap().as_ref().unwrap())), format!("{}{}", "GOPATH=".to_string(), (*self.g_o_p_a_t_h.clone().lock().unwrap().as_ref().unwrap())), format!("{}{}", "CGO_ENABLED=".to_string(), { let __v = (*cgo.lock().unwrap().as_ref().unwrap()).clone(); __v })]); __append_target.clone() }; (*cmd.lock().unwrap().as_mut().unwrap()).env = new_val; };
        {
        let mut err = { let __recv = cmd.clone(); let __recv_ptr: *mut exec_Cmd = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut exec_Cmd }; let __result = unsafe { &mut *__recv_ptr }.run(); __result };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("go/build: go list {}: {}\n{}\n", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*err.lock().unwrap().as_ref().unwrap())), (*(*stderr.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()))))));;
        }
    }
        let mut f = strings::split_n((*stdout.lock().unwrap().as_ref().unwrap()).string(), Arc::new(Mutex::new(Some("\n".to_string()))), Arc::new(Mutex::new(Some(5))));
        if { let __tmp_x = ((*f.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 5; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("go/build: importGo {}: unexpected output:\n{}\n", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*(*stdout.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()))))));
    }
        let mut dir = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = f.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        let mut errStr = strings::trim_space(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = f.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() }))));
        if { let __tmp_x = (*errStr.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = (*dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
                // If 'go list' could not locate the package (dir is empty),
                // return the same error that 'go list' reported.
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from((*errStr.lock().unwrap().as_ref().unwrap()).clone()))));
    }
                // If 'go list' could not locate the package (dir is empty),
                // return the same error that 'go list' reported.
                // If 'go list' did locate the package, ignore the error.
                // It was probably related to loading source files, and we'll
                // encounter it ourselves shortly if the FindOnly flag isn't set.
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };
        { let new_val = { let __seq = { let __seq_holder = f.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; *(*p.lock().unwrap().as_ref().unwrap()).import_path.lock().unwrap() = Some(new_val); };
        { let new_val = { let __seq = { let __seq_holder = f.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }; *(*p.lock().unwrap().as_ref().unwrap()).root.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __seq = { let __seq_holder = f.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() }; let __tmp_y = "true".to_string(); __tmp_x == __tmp_y }; *(*p.lock().unwrap().as_ref().unwrap()).goroot.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(None));
    }

    /// MatchFile reports whether the file with the given name in the given directory
    /// matches the context and would be included in a [Package] created by [ImportDir]
    /// of that directory.
    ///
    /// MatchFile considers the name of the file and may use ctxt.OpenFile to
    /// read some or all of the file's content.
    pub fn match_file(&self, dir: Arc<Mutex<Option<String>>>, name: Arc<Mutex<Option<String>>>) -> (bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut r#match: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let (mut info, __tmp_1) = self.match_file_1(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        return ({ let __nil_result = (*info.lock().unwrap()).is_some(); __nil_result }, err.clone());
    }

    /// matchFile determines whether the file with the given name in the given directory
    /// should be included in the package being constructed.
    /// If the file should be included, matchFile returns a non-nil *fileInfo (and a nil error).
    /// Non-nil errors are reserved for unexpected problems.
    ///
    /// If name denotes a Go program, matchFile reads until the end of the
    /// imports and returns that section of the file in the fileInfo's header field,
    /// even though it only considers text until the first non-comment
    /// for go:build lines.
    ///
    /// If allTags is non-nil, matchFile records any encountered build tag
    /// by setting allTags[tag] = true.
    pub fn match_file_1(&self, dir: Arc<Mutex<Option<String>>>, name: Arc<Mutex<Option<String>>>, allTags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>, mut binaryOnly: Arc<Mutex<Option<bool>>>, fset: Arc<Mutex<Option<token_FileSet>>>) -> (Arc<Mutex<Option<fileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_".to_string())))) || strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".".to_string())))) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        let mut i = strings::last_index(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".".to_string()))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = (*name.lock().unwrap().as_ref().unwrap()).len() as i32; i = new_val; };
    }
        let mut ext = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() })));
        if { let __tmp_x = (*ext.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".go".to_string(); __tmp_x != __tmp_y } && { let __nil_result = (*file_list_for_ext(dummyPkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = ext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap()).is_none(); __nil_result } {
                // skip
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
                // skip
        if !self.good_o_s_arch_file(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), allTags.clone()) && !(*self.use_all_files.clone().lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        let mut info = Arc::new(Mutex::new(Some(fileInfo { name: self.join_path(Arc::new(Mutex::new(Some(vec![(*dir.lock().unwrap().as_ref().unwrap()).clone(), (*name.lock().unwrap().as_ref().unwrap()).clone()])))), fset: fset.clone(), ..Default::default() })));
        if { let __tmp_x = (*ext.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".syso".to_string(); __tmp_x == __tmp_y } {
                // binary, no reading
        return (info.clone(), Arc::new(Mutex::new(None)));
    }
                // binary, no reading
        let (mut f, mut err) = self.open_file(Arc::new(Mutex::new(Some({ let __selector_holder = (*info.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
        if strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".go".to_string())))) {
        { let __rhs_holder = read_go_info({ let __arg = f.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<io_Reader> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, info.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        if strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_test.go".to_string())))) {
        *binaryOnly.lock().unwrap() = None;
    }
    } else {
        *binaryOnly.lock().unwrap() = None;
        { let (__tmp_0, __tmp_1) = read_comments({ let __arg = f.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<io_Reader> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*info.lock().unwrap().as_ref().unwrap()).header.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }
                // ignore //go:binary-only-package comments in _test.go files
                // ignore //go:binary-only-package comments in non-Go sources
        (*f.lock().unwrap().as_ref().unwrap()).close();
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (info.clone(), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("read {}: {}", (*{ let __field = (*info.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone(), format!("{}", (*err.lock().unwrap().as_ref().unwrap()))))))));
    }
                // Look for go:build comments to accept or reject the file.
        let (mut ok, mut sawBinaryOnly, __tmp_2) = self.should_build({ let __field = (*info.lock().unwrap().as_ref().unwrap()).header.clone(); __field }, allTags.clone()); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}: {}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*err.lock().unwrap().as_ref().unwrap()))))))));
    }
        if !ok && !(*self.use_all_files.clone().lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        if { let __nil_result = (*binaryOnly.lock().unwrap()).is_some(); __nil_result } && sawBinaryOnly {
        { let new_val = true; *binaryOnly.lock().unwrap() = Some(new_val); };
    }
        return (info.clone(), Arc::new(Mutex::new(None)));
    }

    /// shouldBuild reports whether it is okay to use this file,
    /// The rule is that in the file's leading run of // comments
    /// and blank lines, which must be followed by a blank line
    /// (to avoid including a Go package clause doc comment),
    /// lines beginning with '//go:build' are taken as build directives.
    ///
    /// The file is accepted only if each such line lists something
    /// matching the file. For example:
    ///
    ///	//go:build windows linux
    ///
    /// marks the file as applicable only on Windows and Linux.
    ///
    /// For each build tag it consults, shouldBuild sets allTags[tag] = true.
    ///
    /// shouldBuild reports whether the file should be built
    /// and whether a //go:binary-only-package comment was found.
    pub fn should_build(&self, mut content: Arc<Mutex<Option<Vec<u8>>>>, allTags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>) -> (bool, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut shouldBuild: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut binaryOnly: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

                // Identify leading run of // comments and blank lines,
                // which must be followed by a blank line.
                // Also identify any //go:build comments.
        let (__tmp_0, mut goBuild, mut sawBinaryOnly, __tmp_3) = parse_file_header(content.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *content.lock().unwrap() = __moved_tmp_0; let __moved_tmp_3 = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_3;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (false, false, err.clone());
    }
                // If //go:build line is present, it controls.
                // Otherwise fall back to +build processing.
        if { let __nil_result = (*goBuild.lock().unwrap()).is_some(); __nil_result } {
            let (mut x, mut err) = constraint::parse(Arc::new(Mutex::new(Some(String::from_utf8((*goBuild.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))));
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (false, false, Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("parsing //go:build line: {}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))))))));
    }
            { let new_val = self.eval(x.clone(), allTags.clone()); *shouldBuild.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = true; *shouldBuild.lock().unwrap() = Some(new_val); };
            let mut p = Arc::new(Mutex::new(Some({ let __v = (*content.lock().unwrap().as_ref().unwrap()).clone(); __v })));
            while { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut line = Arc::new(Mutex::new(Some({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        {
        let mut i = bytes::index_byte(line.clone(), Arc::new(Mutex::new(Some(('\n' as i32) as u8))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let __tmp_0 = Arc::new(Mutex::new(Some({ let __seq_holder = line.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); let __tmp_1 = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); *line.lock().unwrap() = __tmp_0.lock().unwrap().take(); *p.lock().unwrap() = __tmp_1.lock().unwrap().take(); };;
        } else {
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };;
        }
    }
        { let new_val = bytes::trim_space(line.clone()); line = new_val; };
        if !bytes::has_prefix(line.clone(), slashSlash.clone()) || !bytes::contains(line.clone(), plusBuild.clone()) {
        continue
    }
        let mut text = Arc::new(Mutex::new(Some(String::from_utf8((*line.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
        if !constraint::is_plus_build({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) {
        continue
    }
        {
        let (mut x, mut err) = constraint::parse({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            if !self.eval(x.clone(), allTags.clone()) {
        { let new_val = false; *shouldBuild.lock().unwrap() = Some(new_val); };
    };
        }
    }
    }
        }
        return ({ let __v = (*shouldBuild.lock().unwrap().as_ref().unwrap()).clone(); __v }, sawBinaryOnly, Arc::new(Mutex::new(None)));
    }

    /// saveCgo saves the information from the #cgo lines in the import "C" comment.
    /// These lines set CFLAGS, CPPFLAGS, CXXFLAGS and LDFLAGS and pkg-config directives
    /// that affect the way cgo's C code is built.
    pub fn save_cgo(&self, filename: Arc<Mutex<Option<String>>>, di: Arc<Mutex<Option<Package>>>, cg: Arc<Mutex<Option<ast_CommentGroup>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut text = { let __recv = cg.clone(); let __recv_ptr: *mut ast_CommentGroup = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut ast_CommentGroup }; let __result = unsafe { &mut *__recv_ptr }.text(); __result };
        { let __range_holder = strings::split(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("\n".to_string())))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut line in __range_values.iter().cloned() {
        let mut orig = Arc::new(Mutex::new(Some(line.clone())));
                // Line is
                //	#cgo [GOOS/GOARCH...] LDFLAGS: stuff
                //
        { let new_val = (*strings::trim_space(Arc::new(Mutex::new(Some(line.clone())))).lock().unwrap().as_ref().unwrap()).clone(); line = new_val; };
        if { let __tmp_x = (line.len() as i32); let __tmp_y = 5; __tmp_x < __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &(line); let __high = (4) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "#cgo".to_string(); __tmp_x != __tmp_y } || ({ let __tmp_x = { let __s = &(line); __s.as_bytes()[(4) as usize] }; let __tmp_y = (' ' as i32) as u8; __tmp_x != __tmp_y } && { let __tmp_x = { let __s = &(line); __s.as_bytes()[(4) as usize] }; let __tmp_y = ('\t' as i32) as u8; __tmp_x != __tmp_y }) {
        continue
    }
                // #cgo (nocallback|noescape) <function name>
        {
        let mut fields = strings::fields(Arc::new(Mutex::new(Some(line.clone()))));;
        if { let __tmp_x = ((*fields.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 3; __tmp_x == __tmp_y } && ({ let __tmp_x = { let __seq = { let __seq_holder = fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __tmp_y = "nocallback".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __tmp_y = "noescape".to_string(); __tmp_x == __tmp_y }) {
            continue;
        }
    }
                // Split at colon.
        let (mut line, mut argstr, mut ok) = strings::cut(strings::trim_space(Arc::new(Mutex::new(Some({ let __s = &(line); let __low = (4) as usize; __s[__low..].to_string() })))), Arc::new(Mutex::new(Some(":".to_string()))));
        if !ok {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}: invalid #cgo line: {}", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*orig.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
    }
                // Parse GOOS/GOARCH stuff.
        let mut f = strings::fields(Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ((*f.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}: invalid #cgo line: {}", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*orig.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
    }
        let (mut cond, mut verb) = (Arc::new(Mutex::new(Some({ let __seq_holder = f.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = ((*f.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = f.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*f.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }))));
        if { let __tmp_x = ((*cond.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut ok = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = cond.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter() {
        if self.match_auto(Arc::new(Mutex::new(Some((*c).clone()))), Arc::new(Mutex::new(None))) {
        { let new_val = true; *ok.lock().unwrap() = Some(new_val); };
        break
    }
    } }
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        continue
    }
    }
        let (mut args, mut err) = split_quoted(Arc::new(Mutex::new(Some({ let __arg_holder = argstr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}: invalid #cgo line: {}", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*orig.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
    }
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, mut arg) in __range_values.iter().cloned().enumerate() {
        {
        { let (__tmp_0, __tmp_1) = expand_src_dir(Arc::new(Mutex::new(Some(arg.clone()))), Arc::new(Mutex::new(Some({ let __selector_holder = (*di.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); arg = { let __tmp_holder = __tmp_0.clone(); let __tmp_guard = __tmp_holder.lock().unwrap(); (*__tmp_guard.as_ref().unwrap()).clone() }; ok = __tmp_1; };;
        if !ok {
            return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}: malformed #cgo argument: {}", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }, arg)))));;
        }
    }
        (*args.lock().unwrap().as_mut().unwrap())[(i) as usize] = arg;
    } }
        { let _switch_val = (*verb.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("CFLAGS".to_string()) || _switch_val == ("CPPFLAGS".to_string()) || _switch_val == ("CXXFLAGS".to_string()) || _switch_val == ("FFLAGS".to_string()) || _switch_val == ("LDFLAGS".to_string()) {
                        // Change relative paths to absolute.
            self.make_paths_absolute(args.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*di.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        }
    }
                // Change relative paths to absolute.
        { let _switch_val = (*verb.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("CFLAGS".to_string()) {
            { let new_val = { let __append_target = (*di.lock().unwrap().as_ref().unwrap()).cgo_c_f_l_a_g_s.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = args.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*di.lock().unwrap().as_mut().unwrap()).cgo_c_f_l_a_g_s = new_val; };
        } else if _switch_val == ("CPPFLAGS".to_string()) {
            { let new_val = { let __append_target = (*di.lock().unwrap().as_ref().unwrap()).cgo_c_p_p_f_l_a_g_s.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = args.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*di.lock().unwrap().as_mut().unwrap()).cgo_c_p_p_f_l_a_g_s = new_val; };
        } else if _switch_val == ("CXXFLAGS".to_string()) {
            { let new_val = { let __append_target = (*di.lock().unwrap().as_ref().unwrap()).cgo_c_x_x_f_l_a_g_s.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = args.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*di.lock().unwrap().as_mut().unwrap()).cgo_c_x_x_f_l_a_g_s = new_val; };
        } else if _switch_val == ("FFLAGS".to_string()) {
            { let new_val = { let __append_target = (*di.lock().unwrap().as_ref().unwrap()).cgo_f_f_l_a_g_s.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = args.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*di.lock().unwrap().as_mut().unwrap()).cgo_f_f_l_a_g_s = new_val; };
        } else if _switch_val == ("LDFLAGS".to_string()) {
            { let new_val = { let __append_target = (*di.lock().unwrap().as_ref().unwrap()).cgo_l_d_f_l_a_g_s.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = args.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*di.lock().unwrap().as_mut().unwrap()).cgo_l_d_f_l_a_g_s = new_val; };
        } else if _switch_val == ("pkg-config".to_string()) {
            { let new_val = { let __append_target = (*di.lock().unwrap().as_ref().unwrap()).cgo_pkg_config.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = args.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*di.lock().unwrap().as_mut().unwrap()).cgo_pkg_config = new_val; };
        } else {
            return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}: invalid #cgo verb: {}", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*orig.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
        }
    }
    } }
                // Line is
                //	#cgo [GOOS/GOARCH...] LDFLAGS: stuff
                //
                // #cgo (nocallback|noescape) <function name>
                // Split at colon.
                // Parse GOOS/GOARCH stuff.
                // Change relative paths to absolute.
        return Arc::new(Mutex::new(None));
    }

    /// makePathsAbsolute looks for compiler options that take paths and
    /// makes them absolute. We do this because through the 1.8 release we
    /// ran the compiler in the package directory, so any relative -I or -L
    /// options would be relative to that directory. In 1.9 we changed to
    /// running the compiler in the build directory, to get consistent
    /// build results (issue #19964). To keep builds working, we change any
    /// relative -I or -L options to be absolute.
    ///
    /// Using filepath.IsAbs and filepath.Join here means the results will be
    /// different on different systems, but that's OK: -I and -L options are
    /// inherently system-dependent.
    pub fn make_paths_absolute(&self, args: Arc<Mutex<Option<Vec<String>>>>, srcDir: Arc<Mutex<Option<String>>>) {
        let mut nextPath = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, arg) in __range_values.iter().enumerate() {
        if { let __v = (*nextPath.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if !path_filepath::is_abs(Arc::new(Mutex::new(Some((*arg).clone())))) {
        (*args.lock().unwrap().as_mut().unwrap())[(i) as usize] = (*path_filepath::join(Arc::new(Mutex::new(Some(vec![(*srcDir.lock().unwrap().as_ref().unwrap()).clone(), (*arg).clone()])))).lock().unwrap().as_ref().unwrap()).clone();
    }
        { let new_val = false; *nextPath.lock().unwrap() = Some(new_val); };
    } else if strings::has_prefix(Arc::new(Mutex::new(Some((*arg).clone()))), Arc::new(Mutex::new(Some("-I".to_string())))) || strings::has_prefix(Arc::new(Mutex::new(Some((*arg).clone()))), Arc::new(Mutex::new(Some("-L".to_string())))) {
        if { let __tmp_x = (arg.len() as i32); let __tmp_y = 2; __tmp_x == __tmp_y } {
        { let new_val = true; *nextPath.lock().unwrap() = Some(new_val); };
    } else {
        if !path_filepath::is_abs(Arc::new(Mutex::new(Some({ let __s = &(arg); let __low = (2) as usize; __s[__low..].to_string() })))) {
        (*args.lock().unwrap().as_mut().unwrap())[(i) as usize] = format!("{}{}", (*Arc::new(Mutex::new(Some({ let __s = &(arg); let __high = (2) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()), (*path_filepath::join(Arc::new(Mutex::new(Some(vec![(*srcDir.lock().unwrap().as_ref().unwrap()).clone(), { let __s = &(arg); let __low = (2) as usize; __s[__low..].to_string() }])))).lock().unwrap().as_ref().unwrap()));
    }
    }
    }
    } }
    }

    /// matchAuto interprets text as either a +build or //go:build expression (whichever works),
    /// reporting whether the expression matches the build context.
    ///
    /// matchAuto is only used for testing of tag evaluation
    /// and in #cgo lines, which accept either syntax.
    pub fn match_auto(&self, mut text: Arc<Mutex<Option<String>>>, allTags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>) -> bool {
        if strings::contains_any(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("&|()".to_string())))) {
        { let new_val = format!("{}{}", "//go:build ".to_string(), { let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v }); *text.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = format!("{}{}", "// +build ".to_string(), { let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v }); *text.lock().unwrap() = Some(new_val); };
    }
        let (mut x, mut err) = constraint::parse({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return false;
    }
        return self.eval(x.clone(), allTags.clone());
    }

    pub fn eval(&self, x: Arc<Mutex<Option<constraint_Expr>>>, allTags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>) -> bool {
        let allTags_closure_clone = allTags.clone(); let mut ctxt_closure_clone = (*self).clone(); return (*x.lock().unwrap().as_ref().unwrap()).eval(Arc::new(Mutex::new(Some(Box::new(move |tag: Arc<Mutex<Option<String>>>| -> bool {
        ctxt_closure_clone.match_tag(Arc::new(Mutex::new(Some({ let __arg_holder = tag.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), allTags_closure_clone.clone())
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> bool + Send + Sync>))));
    }

    /// matchTag reports whether the name is one of:
    ///
    ///	cgo (if cgo is enabled)
    ///	$GOOS
    ///	$GOARCH
    ///	ctxt.Compiler
    ///	linux (if GOOS = android)
    ///	solaris (if GOOS = illumos)
    ///	darwin (if GOOS = ios)
    ///	unix (if this is a Unix GOOS)
    ///	boringcrypto (if GOEXPERIMENT=boringcrypto is enabled)
    ///	tag (if tag is listed in ctxt.BuildTags, ctxt.ToolTags, or ctxt.ReleaseTags)
    ///
    /// It records all consulted tags in allTags.
    pub fn match_tag(&self, mut name: Arc<Mutex<Option<String>>>, allTags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>) -> bool {
        if { let __nil_result = (*allTags.lock().unwrap()).is_some(); __nil_result } {
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(true))); (*allTags.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
                // special tags
        if (*self.cgo_enabled.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "cgo".to_string(); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.g_o_o_s.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } || { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.g_o_a_r_c_h.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } || { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*self.compiler.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = (*self.g_o_o_s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "android".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "linux".to_string(); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = (*self.g_o_o_s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "illumos".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "solaris".to_string(); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = (*self.g_o_o_s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "unix".to_string(); __tmp_x == __tmp_y } && { let __map = { let __map_holder = syslist::UnixOS().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = self.g_o_o_s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        return true;
    }
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "boringcrypto".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "goexperiment.boringcrypto".to_string(); *name.lock().unwrap() = Some(new_val); };
    }
                // boringcrypto is an old name for goexperiment.boringcrypto
                // other tags
        { let __range_holder = self.build_tags.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tag in __range_values.iter() {
        if { let __tmp_x = (*tag).clone(); let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return true;
    }
    } }
        { let __range_holder = self.tool_tags.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tag in __range_values.iter() {
        if { let __tmp_x = (*tag).clone(); let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return true;
    }
    } }
        { let __range_holder = self.release_tags.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tag in __range_values.iter() {
        if { let __tmp_x = (*tag).clone(); let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return true;
    }
    } }
        false
    }

    /// goodOSArchFile returns false if the name contains a $GOOS or $GOARCH
    /// suffix which does not match the current system.
    /// The recognized name formats are:
    ///
    ///	name_$(GOOS).*
    ///	name_$(GOARCH).*
    ///	name_$(GOOS)_$(GOARCH).*
    ///	name_$(GOOS)_test.*
    ///	name_$(GOARCH)_test.*
    ///	name_$(GOOS)_$(GOARCH)_test.*
    ///
    /// Exceptions:
    /// if GOOS=android, then files with GOOS=linux are also matched.
    /// if GOOS=illumos, then files with GOOS=solaris are also matched.
    /// if GOOS=ios, then files with GOOS=darwin are also matched.
    pub fn good_o_s_arch_file(&self, mut name: Arc<Mutex<Option<String>>>, allTags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>) -> bool {
        { let (__tmp_0, __tmp_1, __tmp_2) = strings::cut(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".".to_string())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_tmp_0; };
                // Before Go 1.4, a file called "linux.go" would be equivalent to having a
                // build tag "linux" in that file. For Go 1.4 and beyond, we require this
                // auto-tagging to apply only to files with a non-empty prefix, so
                // "foo_linux.go" is tagged but "linux.go" is not. This allows new operating
                // systems, such as android, to arrive without breaking existing code with
                // innocuous source code in "android.go". The easiest fix: cut everything
                // in the name before the initial _.
        let mut i = strings::index(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_".to_string()))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return true;
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_val; };
        let mut l = strings::split(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_".to_string()))));
        {
        let mut n = Arc::new(Mutex::new(Some((*l.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = "test".to_string(); __tmp_x == __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); l = new_val; };;
        }
    }
        let mut n = Arc::new(Mutex::new(Some((*l.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x >= __tmp_y } && { let __map = { let __map_holder = syslist::KnownOS().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } && { let __map = { let __map_holder = syslist::KnownArch().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        if { let __nil_result = (*allTags.lock().unwrap()).is_some(); __nil_result } {
                // In case we short-circuit on l[n-1].
        { let __map_key = { let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() }; let __map_value = Arc::new(Mutex::new(Some(true))); (*allTags.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
                // In case we short-circuit on l[n-1].
        return self.match_tag(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }))), allTags.clone()) && self.match_tag(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() }))), allTags.clone());
    }
                // In case we short-circuit on l[n-1].
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } && ({ let __map = { let __map_holder = syslist::KnownOS().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } || { let __map = { let __map_holder = syslist::KnownArch().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) }) {
        return self.match_tag(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = l.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }))), allTags.clone());
    }
        true
    }
}

impl Package {
    /// IsCommand reports whether the package is considered a
    /// command to be installed (not just a library).
    /// Packages named "main" are treated as commands.
    pub fn is_command(&self) -> bool {
        return { let __tmp_x = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "main".to_string(); __tmp_x == __tmp_y };
    }
}

impl NoGoError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "no buildable Go source files in ".to_string(), (*self.dir.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for NoGoError {}


impl MultiplePackageError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
                // Error string limited to two entries for compatibility.
        Arc::new(Mutex::new(Some(format!("found packages {} ({}) and {} ({}) in {}", { let __seq = { let __seq_holder = self.packages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }, { let __seq = { let __seq_holder = self.files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }, { let __seq = { let __seq_holder = self.packages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }, { let __seq = { let __seq_holder = self.files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }, (*self.dir.lock().unwrap().as_ref().unwrap())))))
    }
}

impl StdError for MultiplePackageError {}


/// hasSubdir reports if dir is within root by performing lexical analysis only.
pub fn has_subdir(mut root: Arc<Mutex<Option<String>>>, mut dir: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
    let mut rel: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    const sep: &'static str = "/";

    { let new_val = path_filepath::clean(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *root.lock().unwrap() = __moved_val; };
    if !strings::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/".to_string())))) {
        { (*root.lock().unwrap().as_mut().unwrap()).push_str(&sep); };
    }
    { let new_val = path_filepath::clean(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dir.lock().unwrap() = __moved_val; };
    let (mut after, mut found) = strings::cut_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = root.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !found {
        return (Arc::new(Mutex::new(Some("".to_string()))), false);
    }
    return (path_filepath::to_slash(Arc::new(Mutex::new(Some({ let __arg_holder = after.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), true);
}

/// Keep consistent with cmd/go/internal/cfg.defaultGOPATH.
pub fn default_g_o_p_a_t_h() -> Arc<Mutex<Option<String>>> {
    let mut env = Arc::new(Mutex::new(Some("HOME".to_string())));
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "USERPROFILE".to_string(); *env.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = "darwin".to_string(); let __tmp_y = "plan9".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "home".to_string(); *env.lock().unwrap() = Some(new_val); };
    }
    {
        let mut home = os::getenv({ let __arg_holder = env.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });;
        if { let __tmp_x = (*home.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            let mut def = path_filepath::join(Arc::new(Mutex::new(Some(vec![(*home.lock().unwrap().as_ref().unwrap()).clone(), "go".to_string()]))));;
            if { let __tmp_x = (*path_filepath::clean(Arc::new(Mutex::new(Some({ let __arg_holder = def.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*path_filepath::clean(runtime::g_o_r_o_o_t()).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    };
            return { let __owned = def.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
        // Don't set the default GOPATH to GOROOT,
        // as that will trigger warnings from the go tool.
    Arc::new(Mutex::new(Some("".to_string())))
}

pub fn default_context() -> Arc<Mutex<Option<Context>>> {
    let mut c: Arc<Mutex<Option<Context>>> = Arc::new(Mutex::new(Some(Default::default())));

    { let new_val = { let __selector_holder = buildcfg::GOARCH().clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*c.lock().unwrap().as_ref().unwrap()).g_o_a_r_c_h.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = buildcfg::GOOS().clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*c.lock().unwrap().as_ref().unwrap()).g_o_o_s.lock().unwrap() = Some(new_val); };
    {
        let mut goroot = runtime::g_o_r_o_o_t();;
        if { let __tmp_x = (*goroot.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            { let new_val = path_filepath::clean(Arc::new(Mutex::new(Some({ let __arg_holder = goroot.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*c.lock().unwrap().as_ref().unwrap()).g_o_r_o_o_t.lock().unwrap() = __moved_val; };;
        }
    }
    { let new_val = env_or(Arc::new(Mutex::new(Some("GOPATH".to_string()))), default_g_o_p_a_t_h()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*c.lock().unwrap().as_ref().unwrap()).g_o_p_a_t_h.lock().unwrap() = __moved_val; };
    { let new_val = "gc".to_string(); *(*c.lock().unwrap().as_ref().unwrap()).compiler.lock().unwrap() = Some(new_val); };
    { let new_val = { let __append_target = (*c.lock().unwrap().as_ref().unwrap()).tool_tags.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = buildcfg::ToolTags().clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; (*c.lock().unwrap().as_mut().unwrap()).tool_tags = new_val; };

    { let new_val = { let __collection_holder = { let __append_target = Arc::new(Mutex::new(Some(Vec::<String>::new()))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = (*c.lock().unwrap().as_ref().unwrap()).tool_tags.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *defaultToolTags.lock().unwrap() = new_val; };

        // Each major Go release in the Go 1.x series adds a new
        // "go1.x" release tag. That is, the go1.x tag is present in
        // all releases >= Go 1.x. Code that requires Go 1.x or later
        // should say "go:build go1.x", and code that should only be
        // built before Go 1.x (perhaps it is the stub to use in that
        // case) should say "go:build !go1.x".
        // The last element in ReleaseTags is the current release.
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = goversion::VERSION; __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = (*c.lock().unwrap().as_ref().unwrap()).release_tags.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(format!("{}{}", "go1.".to_string(), (*Arc::new(Mutex::new(Some(((*i.lock().unwrap().as_ref().unwrap())).to_string()))).lock().unwrap().as_ref().unwrap()))); __append_target.clone() }; (*c.lock().unwrap().as_mut().unwrap()).release_tags = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    { let new_val = { let __collection_holder = { let __append_target = Arc::new(Mutex::new(Some(Vec::<String>::new()))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = (*c.lock().unwrap().as_ref().unwrap()).release_tags.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *defaultReleaseTags.lock().unwrap() = new_val; };

    let mut env = os::getenv("CGO_ENABLED".to_string());
    if { let __tmp_x = (*env.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "".to_string(); *env.lock().unwrap() = Some(new_val); };
    }
    '__go_switch_1: loop {
        { let _switch_val = (*env.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("1".to_string()) {
            { let new_val = true; *(*c.lock().unwrap().as_ref().unwrap()).cgo_enabled.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ("0".to_string()) {
            { let new_val = false; *(*c.lock().unwrap().as_ref().unwrap()).cgo_enabled.lock().unwrap() = Some(new_val); };
        } else {
                        // cgo must be explicitly enabled for cross compilation builds
            if { let __tmp_x = "arm64".to_string(); let __tmp_y = { let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).g_o_a_r_c_h.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y } && { let __tmp_x = "darwin".to_string(); let __tmp_y = { let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).g_o_o_s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y } {
        { let new_val = platform::cgo_supported({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).g_o_o_s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).g_o_a_r_c_h.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }); *(*c.lock().unwrap().as_ref().unwrap()).cgo_enabled.lock().unwrap() = Some(new_val); };
        break '__go_switch_1
    }
            { let new_val = false; *(*c.lock().unwrap().as_ref().unwrap()).cgo_enabled.lock().unwrap() = Some(new_val); };
        }
    };
        break;
    }

        // cgo must be explicitly enabled for cross compilation builds
    return { let __owned = c.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn env_or(name: Arc<Mutex<Option<String>>>, def: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut s = os::getenv({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return { let __owned = def.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn name_ext(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut i = strings::last_index(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".".to_string()))));
    if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() })))
}

pub fn file_list_for_ext(p: Arc<Mutex<Option<Package>>>, ext: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    { let _switch_val = (*ext.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (".c".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).c_files.clone();
        } else if _switch_val == (".cc".to_string()) || _switch_val == (".cpp".to_string()) || _switch_val == (".cxx".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).c_x_x_files.clone();
        } else if _switch_val == (".m".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).m_files.clone();
        } else if _switch_val == (".h".to_string()) || _switch_val == (".hh".to_string()) || _switch_val == (".hpp".to_string()) || _switch_val == (".hxx".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).h_files.clone();
        } else if _switch_val == (".f".to_string()) || _switch_val == (".F".to_string()) || _switch_val == (".for".to_string()) || _switch_val == (".f90".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).f_files.clone();
        } else if _switch_val == (".s".to_string()) || _switch_val == (".S".to_string()) || _switch_val == (".sx".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).s_files.clone();
        } else if _switch_val == (".swig".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).swig_files.clone();
        } else if _switch_val == (".swigcxx".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).swig_c_x_x_files.clone();
        } else if _switch_val == (".syso".to_string()) {
            return (*p.lock().unwrap().as_ref().unwrap()).syso_files.clone();
        }
    }
    return Arc::new(Mutex::new(None));
}

pub fn equal(x: Arc<Mutex<Option<Vec<String>>>>, y: Arc<Mutex<Option<Vec<String>>>>) -> bool {
    if { let __tmp_x = ((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*y.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        return false;
    }
    { let __range_holder = x.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, xi) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*xi).clone(); let __tmp_y = { let __seq = { let __seq_holder = y.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; __tmp_x != __tmp_y } {
        return false;
    }
    } }
    true
}

/// hasGoFiles reports whether dir contains any files with names ending in .go.
/// For a vendor check we must exclude directories that contain no .go files.
/// Otherwise it is not possible to vendor just a/b/c and still import the
/// non-vendored a/b. See golang.org/issue/13832.
pub fn has_go_files(ctxt: Arc<Mutex<Option<Context>>>, dir: Arc<Mutex<Option<String>>>) -> bool {
    let (mut ents, _) = { let __recv = ctxt.clone(); let __recv_ptr: *const Context = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Context }; let __result = unsafe { &*__recv_ptr }.read_dir(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    { let __range_holder = ents.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ent in __range_values.iter() {
        if !ent.is_dir() && strings::has_suffix(ent.name(), Arc::new(Mutex::new(Some(".go".to_string())))) {
        return true;
    }
    } }
    false
}

pub fn find_import_comment(mut data: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<String>>>, i32) {
    let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        // expect keyword package
    let (mut word, __tmp_1) = parse_word(data.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *data.lock().unwrap() = __moved_tmp_1;;
    if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*word.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "package".to_string(); __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0);
    }

        // expect package name
    { let (__tmp_0, __tmp_1) = parse_word(data.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *data.lock().unwrap() = __moved_tmp_1; };

        // now ready for import comment, a // or /* */ comment
        // beginning and ending on the current line.
    while { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && ({ let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = ('\t' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = ('\r' as i32) as u8; __tmp_x == __tmp_y }) {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
    }

    let mut comment: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    if bytes::has_prefix(data.clone(), slashSlash.clone()) {
            { let (__tmp_0, __tmp_1, __tmp_2) = bytes::cut(Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (2) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), newline.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *comment.lock().unwrap() = __moved_tmp_0; };
        } else if bytes::has_prefix(data.clone(), slashStar.clone()) {
            let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
            { let (__tmp_0, __tmp_1, __tmp_2) = bytes::cut(Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (2) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), starSlash.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *comment.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_2); };
            if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // malformed comment
        return (Arc::new(Mutex::new(Some("".to_string()))), 0);
    }
                        // malformed comment
            if bytes::contains(comment.clone(), newline.clone()) {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0);
    }
        }
        // malformed comment
    { let new_val = bytes::trim_space(comment.clone()); comment = new_val; };

        // split comment into `import`, `"pkg"`
    let (__tmp_0, mut arg) = parse_word(comment.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *word.lock().unwrap() = __moved_tmp_0;;
    if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*word.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "import".to_string(); __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), 0);
    }

    { let new_val = { let __tmp_x = 1; let __tmp_y = bytes::count(Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); let __tmp_y = ((*arg.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), newline.clone()); __tmp_x + __tmp_y }; *line.lock().unwrap() = Some(new_val); };
    return (strings::trim_space(Arc::new(Mutex::new(Some(String::from_utf8((*arg.lock().unwrap().as_ref().unwrap()).clone()).unwrap())))), { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// skipSpaceOrComment returns data with any leading spaces or comments removed.
pub fn skip_space_or_comment(mut data: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    while { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let _switch_val = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
    if _switch_val == ((' ' as i32) as u8) || _switch_val == (('\t' as i32) as u8) || _switch_val == (('\r' as i32) as u8) || _switch_val == (('\n' as i32) as u8) {
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
            continue
        } else if _switch_val == (('/' as i32) as u8) {
            if bytes::has_prefix(data.clone(), slashSlash.clone()) {
        let mut i = bytes::index(data.clone(), newline.clone());
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
        continue
    }
            if bytes::has_prefix(data.clone(), slashStar.clone()) {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (2) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
        let mut i = bytes::index(data.clone(), starSlash.clone());
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = i; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
        continue
    }
        }
    }
        break
    }
    return data.clone();
}

/// parseWord skips any leading spaces or comments in data
/// and then parses the beginning of data as an identifier or keyword,
/// returning that word and what remains after the word.
pub fn parse_word(mut data: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Vec<u8>>>>) {
    let mut word: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut rest: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));

    { let new_val = skip_space_or_comment(data.clone()); data = new_val; };

        // Parse past leading word characters.
    { let new_val = data.clone(); rest = new_val; };
    loop {
        let (mut r, mut size) = unicode_utf8::decode_rune(rest.clone());
        if unicode::is_letter(Arc::new(Mutex::new(Some(r)))) || { let __tmp_x = ('0' as i32); let __tmp_y = r; __tmp_x <= __tmp_y } && { let __tmp_x = r; let __tmp_y = ('9' as i32); __tmp_x <= __tmp_y } || { let __tmp_x = r; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = rest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (size) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); rest = new_val; };
        continue
    }
        break
    }

    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*rest.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); word = new_val; };
    if { let __tmp_x = ((*word.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }

    return (word.clone(), rest.clone());
}

pub fn clean_decls(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<token_Position>>>>>>>>) {
    let mut all = Arc::new(Mutex::new(Some(Vec::<String>::with_capacity(((*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (path, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = all.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(path.clone()); __append_target.clone() }; all = new_val; };
    }
    { let __sort_target = all.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    return (all.clone(), m.clone());
}

pub fn is_go_build_comment(mut line: Arc<Mutex<Option<Vec<u8>>>>) -> bool {
    if !bytes::has_prefix(line.clone(), goBuildComment.clone()) {
        return false;
    }
    { let new_val = bytes::trim_space(line.clone()); line = new_val; };
    let mut rest = Arc::new(Mutex::new(Some({ let __seq_holder = line.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ((*goBuildComment.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    return { let __tmp_x = ((*rest.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = ((*bytes::trim_space(rest.clone()).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*rest.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y };
}

/// parseFileHeader should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bazelbuild/bazel-gazelle
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname parseFileHeader
pub fn parse_file_header(content: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Vec<u8>>>>, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut trimmed: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut goBuild: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut sawBinaryOnly: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut end = Arc::new(Mutex::new(Some(0)));
    let mut p = Arc::new(Mutex::new(Some({ let __v = (*content.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    let mut ended = Arc::new(Mutex::new(Some(false)));
    let mut inSlashStar = Arc::new(Mutex::new(Some(false)));

    'lines: while { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut line = Arc::new(Mutex::new(Some({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        {
        let mut i = bytes::index_byte(line.clone(), Arc::new(Mutex::new(Some(('\n' as i32) as u8))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let __tmp_0 = Arc::new(Mutex::new(Some({ let __seq_holder = line.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); let __tmp_1 = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); *line.lock().unwrap() = __tmp_0.lock().unwrap().take(); *p.lock().unwrap() = __tmp_1.lock().unwrap().take(); };;
        } else {
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };;
        }
    }
        { let new_val = bytes::trim_space(line.clone()); line = new_val; };
        if { let __tmp_x = ((*line.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && !{ let __v = (*ended.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Remember position of most recent blank line.
                // When we find the first non-blank, non-// line,
                // this "end" position marks the latest file position
                // where a //go:build line can appear.
                // (It must appear _before_ a blank line before the non-blank, non-// line.
                // Yes, that's confusing, which is part of why we moved to //go:build lines.)
                // Note that ended==false here means that inSlashStar==false,
                // since seeing a /* would have set ended==true.
        { let new_val = { let __tmp_x = ((*content.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x - __tmp_y }; *end.lock().unwrap() = Some(new_val); };
        continue 'lines
    }
                // Remember position of most recent blank line.
                // When we find the first non-blank, non-// line,
                // this "end" position marks the latest file position
                // where a //go:build line can appear.
                // (It must appear _before_ a blank line before the non-blank, non-// line.
                // Yes, that's confusing, which is part of why we moved to //go:build lines.)
                // Note that ended==false here means that inSlashStar==false,
                // since seeing a /* would have set ended==true.
        if !bytes::has_prefix(line.clone(), slashSlash.clone()) {
        { let new_val = true; *ended.lock().unwrap() = Some(new_val); };
    }

        if !{ let __v = (*inSlashStar.lock().unwrap().as_ref().unwrap()).clone(); __v } && is_go_build_comment(line.clone()) {
        if { let __nil_result = (*goBuild.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), false, errMultipleGoBuild.clone());
    }
        { let new_val = line.clone(); goBuild = new_val; };
    }
        if !{ let __v = (*inSlashStar.lock().unwrap().as_ref().unwrap()).clone(); __v } && bytes::equal(line.clone(), binaryOnlyComment.clone()) {
        { let new_val = true; *sawBinaryOnly.lock().unwrap() = Some(new_val); };
    }

        'comments: while { let __tmp_x = ((*line.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __v = (*inSlashStar.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let mut i = bytes::index(line.clone(), starSlash.clone());;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let new_val = false; *inSlashStar.lock().unwrap() = Some(new_val); };;
            { let new_val = bytes::trim_space(Arc::new(Mutex::new(Some({ let __seq_holder = line.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = (i as i32); let __tmp_y = ((*starSlash.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); line = new_val; };;
            continue 'comments;
        }
    }
        continue 'lines
    }
        if bytes::has_prefix(line.clone(), slashSlash.clone()) {
        continue 'lines
    }
        if bytes::has_prefix(line.clone(), slashStar.clone()) {
        { let new_val = true; *inSlashStar.lock().unwrap() = Some(new_val); };
        { let new_val = bytes::trim_space(Arc::new(Mutex::new(Some({ let __seq_holder = line.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ((*slashStar.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); line = new_val; };
        continue 'comments
    }

                // Found non-comment text.
        break 'lines
    }
    }

        // Blank line
        // Remember position of most recent blank line.
        // When we find the first non-blank, non-// line,
        // this "end" position marks the latest file position
        // where a //go:build line can appear.
        // (It must appear _before_ a blank line before the non-blank, non-// line.
        // Yes, that's confusing, which is part of why we moved to //go:build lines.)
        // Note that ended==false here means that inSlashStar==false,
        // since seeing a /* would have set ended==true.
        // Not comment line
        // Found non-comment text.
    return (Arc::new(Mutex::new(Some({ let __seq_holder = content.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), goBuild.clone(), { let __v = (*sawBinaryOnly.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
}

/// expandSrcDir expands any occurrence of ${SRCDIR}, making sure
/// the result is safe for the shell.
pub fn expand_src_dir(str: Arc<Mutex<Option<String>>>, mut srcdir: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        // "\" delimited paths cause safeCgoName to fail
        // so convert native paths with a different delimiter
        // to "/" before starting (eg: on windows).
    { let new_val = path_filepath::to_slash(Arc::new(Mutex::new(Some({ let __arg_holder = srcdir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *srcdir.lock().unwrap() = __moved_val; };

    let mut chunks = strings::split(Arc::new(Mutex::new(Some({ let __arg_holder = str.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("${SRCDIR}".to_string()))));
    if { let __tmp_x = ((*chunks.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        return ({ let __owned = str.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, safe_cgo_name(Arc::new(Mutex::new(Some({ let __arg_holder = str.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
    }
    let mut ok = Arc::new(Mutex::new(Some(true)));
    { let __range_holder = chunks.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for chunk in __range_values.iter() {
        { let new_val = { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } && ({ let __tmp_x = (*chunk).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } || safe_cgo_name(Arc::new(Mutex::new(Some((*chunk).clone()))))); *ok.lock().unwrap() = Some(new_val); };
    } }
    { let new_val = { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } && ({ let __tmp_x = (*srcdir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } || safe_cgo_name(Arc::new(Mutex::new(Some({ let __arg_holder = srcdir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); *ok.lock().unwrap() = Some(new_val); };
    let mut res = strings::join(chunks.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = srcdir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return ({ let __owned = res.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*res.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y });
}

pub fn safe_cgo_name(s: Arc<Mutex<Option<String>>>) -> bool {
    if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return false;
    }
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));;
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x < __tmp_y } && { let __tmp_x = strings::index_byte(Arc::new(Mutex::new(Some("+-.,/0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz:$@%! ~^".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x < __tmp_y } {
            return false;;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    true
}

/// splitQuoted splits the string s around each instance of one or more consecutive
/// white space characters while taking into account quotes and escaping, and
/// returns an array of substrings of s or an empty list if s contains only white space.
/// Single quotes and double quotes are recognized to prevent splitting within the
/// quoted region, and are removed from the resulting substrings. If a quote in s
/// isn't closed err will be set and r will have the unclosed argument as the
/// last element. The backslash is used for escaping.
///
/// For example, the following string:
///
///	a b:"c d" 'e''f'  "g\""
///
/// Would be parsed as:
///
///	[]string{"a", "b:c d", "ef", `g"`}
pub fn split_quoted(s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut r: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut args: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    let mut arg = Arc::new(Mutex::new(Some(vec![0; ((*s.lock().unwrap().as_ref().unwrap()).len()) as usize])));
    let mut escaped = Arc::new(Mutex::new(Some(false)));
    let mut quoted = Arc::new(Mutex::new(Some(false)));
    let mut quote = Arc::new(Mutex::new(Some(('\u{0}' as i32))));
    let mut i = Arc::new(Mutex::new(Some(0)));
    for (_, rune) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __v = (*escaped.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            { let new_val = false; *escaped.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = rune; let __tmp_y = '\\'; __tmp_x == __tmp_y } {
            { let new_val = true; *escaped.lock().unwrap() = Some(new_val); };
            continue
        } else if { let __tmp_x = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\u{0}' as i32); __tmp_x != __tmp_y } {
            if { let __tmp_x = rune as i32; let __tmp_y = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let new_val = ('\u{0}' as i32); *quote.lock().unwrap() = Some(new_val); };
        continue
    }
        } else if { let __tmp_x = rune; let __tmp_y = '"'; __tmp_x == __tmp_y } || { let __tmp_x = rune; let __tmp_y = '\''; __tmp_x == __tmp_y } {
            { let new_val = true; *quoted.lock().unwrap() = Some(new_val); };
            { let new_val = rune as i32; *quote.lock().unwrap() = Some(new_val); };
            continue
        } else if unicode::is_space(Arc::new(Mutex::new(Some(rune as i32)))) {
            if { let __v = (*quoted.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = false; *quoted.lock().unwrap() = Some(new_val); };
        { let new_val = { let __append_target = args.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some({ let __rune_slice_holder = Arc::new(Mutex::new(Some({ let __seq_holder = arg.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __rune_slice_guard = __rune_slice_holder.lock().unwrap(); (*__rune_slice_guard.as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>() }))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; args = new_val; };
        { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    }
            continue
        }
        (*arg.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = rune as i32;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __v = (*quoted.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = args.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some({ let __rune_slice_holder = Arc::new(Mutex::new(Some({ let __seq_holder = arg.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __rune_slice_guard = __rune_slice_holder.lock().unwrap(); (*__rune_slice_guard.as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>() }))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; args = new_val; };
    }
    if { let __tmp_x = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("unclosed quote".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    } else if { let __v = (*escaped.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("unfinished escaping".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return (args.clone(), err.clone());
}

/// IsLocalImport reports whether the import path is
/// a local import path, like ".", "..", "./foo", or "../foo".
pub fn is_local_import(path: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "..".to_string(); __tmp_x == __tmp_y } || strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("./".to_string())))) || strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("../".to_string()))));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub vendor: Arc<Mutex<Option<Vec<String>>>>,
    pub goroot: Arc<Mutex<Option<String>>>,
    pub gopath: Arc<Mutex<Option<Vec<String>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { vendor: self.vendor.clone(), goroot: { let __guard = self.goroot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gopath: self.gopath.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { vendor: Arc::new(Mutex::new(None)), goroot: Arc::new(Mutex::new(Some(String::new()))), gopath: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.vendor), (*self.goroot.lock().unwrap().as_ref().unwrap()), format_slice(&self.gopath))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Context {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Package {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Directive {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for NoGoError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for MultiplePackageError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for fileInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for fileImport {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for fileEmbed {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
