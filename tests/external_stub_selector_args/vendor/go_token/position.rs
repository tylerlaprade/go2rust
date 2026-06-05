use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::serialize::*;
use crate::r#mod::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DEBUG: bool = false;


pub const NO_POS: i32 = 0;


/// Position describes an arbitrary source position
/// including the file, line, and column location.
/// A Position is valid if the line number is > 0.
#[derive(Debug, Clone)]
pub struct Position {
    pub filename: Arc<Mutex<Option<String>>>,
    pub offset: Arc<Mutex<Option<i32>>>,
    pub line: Arc<Mutex<Option<i32>>>,
    pub column: Arc<Mutex<Option<i32>>>,
}

impl Position {
    pub fn __go_value_clone(&self) -> Self {
        Self { filename: { let __guard = self.filename.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, line: { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, column: { let __guard = self.column.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Position {
    fn default() -> Self {
        Self { filename: Arc::new(Mutex::new(Some(String::new()))), offset: Arc::new(Mutex::new(Some(0))), line: Arc::new(Mutex::new(Some(0))), column: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Position {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Filename") {
            out.filename = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Offset") {
            out.offset = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Line") {
            out.line = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Column") {
            out.column = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// Pos is a compact encoding of a source position within a file set.
/// It can be converted into a [Position] for a more convenient, but much
/// larger, representation.
///
/// The Pos value for a given file is a number in the range [base, base+size],
/// where base and size are specified when a file is added to the file set.
/// The difference between a Pos value and the corresponding file base
/// corresponds to the byte offset of that position (represented by the Pos value)
/// from the beginning of the file. Thus, the file base offset is the Pos value
/// representing the first byte in the file.
///
/// To create the Pos value for a specific source offset (measured in bytes),
/// first add the respective file to the current file set using [FileSet.AddFile]
/// and then call [File.Pos](offset) for that file. Given a Pos value p
/// for a specific file set fset, the corresponding [Position] value is
/// obtained by calling fset.Position(p).
///
/// Pos values can be compared directly with the usual comparison operators:
/// If two Pos values p and q are in the same file, comparing p and q is
/// equivalent to comparing the respective source file offsets. If p and q
/// are in different files, p < q is true if the file implied by p was added
/// to the respective file set before the file implied by q.
#[derive(Debug, Clone, Default)]
pub struct Pos(pub Arc<Mutex<Option<i32>>>);

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Pos {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Pos {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Pos> for i32 {
    fn eq(&self, other: &Pos) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Pos> for i32 {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Pos {
    type Output = Pos;
    fn add(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Pos> for i32 {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Pos {
    type Output = Pos;
    fn sub(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Pos> for i32 {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Pos {
    type Output = Pos;
    fn mul(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Pos;
    fn mul(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Pos> for i32 {
    type Output = Pos;
    fn mul(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Pos {
    type Output = Pos;
    fn div(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Pos {
    type Output = Pos;
    fn div(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Pos> for i32 {
    type Output = Pos;
    fn div(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Pos {
    type Output = Pos;
    fn neg(self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Pos {
    type Output = Pos;
    fn rem(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Pos {
    type Output = Pos;
    fn rem(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Pos> for i32 {
    type Output = Pos;
    fn rem(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Pos {
    type Output = Pos;
    fn bitand(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Pos {
    type Output = Pos;
    fn bitand(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Pos> for i32 {
    type Output = Pos;
    fn bitand(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Pos {
    type Output = Pos;
    fn bitor(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Pos {
    type Output = Pos;
    fn bitor(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Pos> for i32 {
    type Output = Pos;
    fn bitor(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Pos {
    type Output = Pos;
    fn bitxor(self, other: Self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Pos {
    type Output = Pos;
    fn bitxor(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Pos> for i32 {
    type Output = Pos;
    fn bitxor(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Pos {
    type Output = Pos;
    fn not(self) -> Pos {
        Pos(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Pos {
    type Output = Pos;
    fn shl(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Pos {
    type Output = Pos;
    fn shl(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Pos {
    type Output = Pos;
    fn shl(self, other: i8) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Pos {
    type Output = Pos;
    fn shl(self, other: i16) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Pos {
    type Output = Pos;
    fn shl(self, other: i64) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Pos {
    type Output = Pos;
    fn shl(self, other: u32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Pos {
    type Output = Pos;
    fn shl(self, other: u8) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Pos {
    type Output = Pos;
    fn shl(self, other: u16) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Pos {
    type Output = Pos;
    fn shl(self, other: u64) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Pos {
    type Output = Pos;
    fn shl(self, other: usize) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Pos {
    type Output = Pos;
    fn shr(self, other: Pos) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Pos {
    type Output = Pos;
    fn shr(self, other: i32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Pos {
    type Output = Pos;
    fn shr(self, other: i8) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Pos {
    type Output = Pos;
    fn shr(self, other: i16) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Pos {
    type Output = Pos;
    fn shr(self, other: i64) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Pos {
    type Output = Pos;
    fn shr(self, other: u32) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Pos {
    type Output = Pos;
    fn shr(self, other: u8) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Pos {
    type Output = Pos;
    fn shr(self, other: u16) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Pos {
    type Output = Pos;
    fn shr(self, other: u64) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Pos {
    type Output = Pos;
    fn shr(self, other: usize) -> Pos {
        Pos(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Pos {}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A File is a handle for a file belonging to a [FileSet].
/// A File has a name, size, and line offset table.
#[derive(Clone)]
pub struct File {
    pub name: Arc<Mutex<Option<String>>>,
    pub base: Arc<Mutex<Option<i32>>>,
    pub size: Arc<Mutex<Option<i32>>>,
    pub mutex: sync::mutex::Mutex,
    pub lines: Arc<Mutex<Option<Vec<i32>>>>,
    pub infos: Arc<Mutex<Option<Vec<lineInfo>>>>,
}

impl File {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mutex: self.mutex.clone(), lines: self.lines.clone(), infos: self.infos.clone() }
    }
}


impl Default for File {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), base: Arc::new(Mutex::new(Some(0))), size: Arc::new(Mutex::new(Some(0))), mutex: Default::default(), lines: Arc::new(Mutex::new(None)), infos: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.base.lock().unwrap().as_ref().unwrap()), (*self.size.lock().unwrap().as_ref().unwrap()), format_slice(&self.lines), format_slice(&self.infos))
    }
}
impl GoComparable for File {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for File {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A lineInfo object describes alternative file, line, and column
/// number information (such as provided via a //line directive)
/// for a given file offset.
#[derive(Debug, Clone)]
pub struct lineInfo {
    pub offset: Arc<Mutex<Option<i32>>>,
    pub filename: Arc<Mutex<Option<String>>>,
    pub line: Arc<Mutex<Option<i32>>>,
    pub column: Arc<Mutex<Option<i32>>>,
}

impl lineInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, filename: { let __guard = self.filename.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, line: { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, column: { let __guard = self.column.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for lineInfo {
    fn default() -> Self {
        Self { offset: Arc::new(Mutex::new(Some(0))), filename: Arc::new(Mutex::new(Some(String::new()))), line: Arc::new(Mutex::new(Some(0))), column: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for lineInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.offset.lock().unwrap().as_ref().unwrap()), (*self.filename.lock().unwrap().as_ref().unwrap()), (*self.line.lock().unwrap().as_ref().unwrap()), (*self.column.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for lineInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Offset") {
            out.offset = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Filename") {
            out.filename = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Line") {
            out.line = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Column") {
            out.column = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A FileSet represents a set of source files.
/// Methods of file sets are synchronized; multiple goroutines
/// may invoke them concurrently.
///
/// The byte offsets for each file in a file set are mapped into
/// distinct (integer) intervals, one interval [base, base+size]
/// per file. [FileSet.Base] represents the first byte in the file, and size
/// is the corresponding file size. A [Pos] value is a value in such
/// an interval. By determining the interval a [Pos] value belongs
/// to, the file, its file base, and thus the byte offset (position)
/// the [Pos] value is representing can be computed.
///
/// When adding a new file, a file base must be provided. That can
/// be any integer value that is past the end of any interval of any
/// file already in the file set. For convenience, [FileSet.Base] provides
/// such a value, which is simply the end of the Pos interval of the most
/// recently added file, plus one. Unless there is a need to extend an
/// interval later, using the [FileSet.Base] should be used as argument
/// for [FileSet.AddFile].
///
/// A [File] may be removed from a FileSet when it is no longer needed.
/// This may reduce memory usage in a long-running application.
#[derive(Clone)]
pub struct FileSet {
    pub mutex: sync::rwmutex::RWMutex,
    pub base: Arc<Mutex<Option<i32>>>,
    pub files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<File>>>>>>>,
    pub last: Arc<Mutex<Option<sync_atomic::r#type::Pointer<File>>>>,
}

impl FileSet {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, files: self.files.clone(), last: { let __guard = self.last.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for FileSet {
    fn default() -> Self {
        Self { mutex: Default::default(), base: Arc::new(Mutex::new(Some(0))), files: Arc::new(Mutex::new(None)), last: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for FileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.files), (*self.last.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for FileSet {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Position {
    /// IsValid reports whether the position is valid.
    pub fn is_valid(&self) -> bool {
        return { let __tmp_x = (*self.line.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y };
    }

    /// String returns a string in one of several forms:
    ///
    ///	file:line:column    valid position with file name
    ///	file:line           valid position with file name but no column (column == 0)
    ///	line:column         valid position without file name
    ///	line                valid position without file name and no column (column == 0)
    ///	file                invalid position with file name
    ///	-                   invalid position without file name
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = Arc::new(Mutex::new(Some({ let __selector_holder = self.filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if self.is_valid() {
        if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&":".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = Arc::new(Mutex::new(Some(({ let __selector_holder = self.line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }).to_string()))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
        if { let __tmp_x = (*self.column.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x != __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = Arc::new(Mutex::new(Some(format!(":{}", (*self.column.lock().unwrap().as_ref().unwrap()))))); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
    }
    }
        if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "-".to_string(); *s.lock().unwrap() = Some(new_val); };
    }
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
}

impl Pos {
    /// IsValid reports whether the position is valid.
    pub fn is_valid(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Pos(Arc::new(Mutex::new(Some(NO_POS as i32)))); __tmp_x != __tmp_y };
    }
}

impl cmp::r#mod::Ordered for Pos {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Pos>() {
            self == __other
        } else {
            false
        }
    }
}

impl File {
    /// Name returns the file name of file f as registered with AddFile.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    /// Base returns the base offset of file f as registered with AddFile.
    pub fn base(&self) -> i32 {
        return (*self.base.lock().unwrap().as_ref().unwrap());
    }

    /// Size returns the size of file f as registered with AddFile.
    pub fn size(&self) -> i32 {
        return (*self.size.lock().unwrap().as_ref().unwrap());
    }

    /// LineCount returns the number of lines in file f.
    pub fn line_count(&self) -> i32 {
        self.mutex.lock();
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.lines.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        self.mutex.unlock();
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// AddLine adds the line offset for a new line.
    /// The line offset must be larger than the offset for the previous line
    /// and smaller than the file size; otherwise the line offset is ignored.
    pub fn add_line(&mut self, offset: Arc<Mutex<Option<i32>>>) {
        self.mutex.lock();
        {
        let mut i = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.lines.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = self.lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) && { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.size.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
            { let new_val = { let __append_target = self.lines.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*offset.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; self.lines = new_val; };;
        }
    }
        self.mutex.unlock();
    }

    /// MergeLine merges a line with the following line. It is akin to replacing
    /// the newline character at the end of the line with a space (to not change the
    /// remaining offsets). To obtain the line number, consult e.g. [Position.Line].
    /// MergeLine will panic if given an invalid line number.
    pub fn merge_line(&mut self, line: Arc<Mutex<Option<i32>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if { let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid line number {} (should be >= 1)", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
            self.mutex.lock();
            let mut f_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        f_defer_captured.mutex.unlock();
    }));
            if { let __tmp_x = ({ let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.lines.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid line number {} (should be < {})", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }, ({ let __len_target = { let __field = self.lines.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
                        // To merge the line numbered <line> with the line numbered <line+1>,
                        // we need to remove the entry in lines corresponding to the line
                        // numbered <line+1>. The entry in lines corresponding to the line
                        // numbered <line+1> is located at index <line>, since indices in lines
                        // are 0-based and line numbers are 1-based.
            { let _dst_start = ({ let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (*self.lines.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = ({ let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __seq[__low..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.lines.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
            { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = self.lines.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }))); self.lines = new_val; };

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                ()
            }
        }
    }

    /// Lines returns the effective line offset table of the form described by [File.SetLines].
    /// Callers must not mutate the result.
    pub fn lines(&self) -> Arc<Mutex<Option<Vec<i32>>>> {
        self.mutex.lock();
        let mut lines = self.lines.clone();
        self.mutex.unlock();
        return lines.clone();
    }

    /// SetLines sets the line offsets for a file and reports whether it succeeded.
    /// The line offsets are the offsets of the first character of each line;
    /// for instance for the content "ab\nc\n" the line offsets are {0, 3}.
    /// An empty file has an empty line offset table.
    /// Each line offset must be larger than the offset for the previous line
    /// and smaller than the file size; otherwise SetLines fails and returns
    /// false.
    /// Callers must not mutate the provided slice after SetLines returns.
    pub fn set_lines(&mut self, lines: Arc<Mutex<Option<Vec<i32>>>>) -> bool {
                // verify validity of lines table
        let mut size = Arc::new(Mutex::new(Some({ let __selector_holder = self.size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let __range_holder = lines.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, offset) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = offset; let __tmp_y = { let __seq = { let __seq_holder = lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __tmp_x <= __tmp_y } || { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = offset; __tmp_x <= __tmp_y } {
        return false;
    }
    } }
                // set lines table
        self.mutex.lock();
        { let new_val = lines.clone(); self.lines = new_val; };
        self.mutex.unlock();
        true
    }

    /// SetLinesForContent sets the line offsets for the given file content.
    /// It ignores position-altering //line comments.
    pub fn set_lines_for_content(&mut self, content: Arc<Mutex<Option<Vec<u8>>>>) {
        let mut lines: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
        let mut line = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = content.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (offset, b) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = lines.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*line.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; lines = new_val; };
    }
        { let new_val = -1; *line.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = b; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = offset as i32; let __tmp_y = 1; __tmp_x + __tmp_y }; *line.lock().unwrap() = Some(new_val); };
    }
    } }
                // set lines table
        self.mutex.lock();
        { let new_val = lines.clone(); self.lines = new_val; };
        self.mutex.unlock();
    }

    /// LineStart returns the [Pos] value of the start of the specified line.
    /// It ignores any alternative positions set using [File.AddLineColumnInfo].
    /// LineStart panics if the 1-based line number is invalid.
    pub fn line_start(&self, line: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Pos>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if { let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid line number {} (should be >= 1)", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
            self.mutex.lock();
            let mut f_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        f_defer_captured.mutex.unlock();
    }));
            if { let __tmp_x = ({ let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.lines.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid line number {} (should be < {})", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }, ({ let __len_target = { let __field = self.lines.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Pos(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __seq = { let __seq_holder = self.lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __tmp_x + __tmp_y } as i32)))))));
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                Arc::new(Mutex::new(Some(Default::default())))
            }
        }
    }

    /// AddLineInfo is like [File.AddLineColumnInfo] with a column = 1 argument.
    /// It is here for backward-compatibility for code prior to Go 1.11.
    pub fn add_line_info(&mut self, offset: Arc<Mutex<Option<i32>>>, filename: Arc<Mutex<Option<String>>>, line: Arc<Mutex<Option<i32>>>) {
        self.add_line_column_info(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1))));
    }

    /// AddLineColumnInfo adds alternative file, line, and column number
    /// information for a given file offset. The offset must be larger
    /// than the offset for the previously added alternative line info
    /// and smaller than the file size; otherwise the information is
    /// ignored.
    ///
    /// AddLineColumnInfo is typically used to register alternative position
    /// information for line directives such as //line filename:line:column.
    pub fn add_line_column_info(&mut self, offset: Arc<Mutex<Option<i32>>>, filename: Arc<Mutex<Option<String>>>, line: Arc<Mutex<Option<i32>>>, column: Arc<Mutex<Option<i32>>>) {
        self.mutex.lock();
        {
        let mut i = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.infos.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __seq = { let __seq_holder = self.infos.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) && { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.size.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
            { let new_val = { let __append_target = self.infos.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(lineInfo { offset: Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), filename: Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), line: Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), column: Arc::new(Mutex::new(Some({ let __arg_holder = column.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }); __append_target.clone() }; self.infos = new_val; };;
        }
    }
        self.mutex.unlock();
    }

    /// fixOffset fixes an out-of-bounds offset such that 0 <= offset <= f.size.
    pub fn fix_offset(&self, offset: Arc<Mutex<Option<i32>>>) -> i32 {
        if { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
            if !DEBUG {
        return 0;
    }
        } else if { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.size.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
            if !DEBUG {
        return (*self.size.lock().unwrap().as_ref().unwrap());
    }
        } else {
            return { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v };
        }
                // only generate this code if needed
        if DEBUG {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("offset {} out of bounds [{}, {}] (position {} out of bounds [{}, {}])", 0, { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*self.size.lock().unwrap().as_ref().unwrap()), { let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }, (*self.base.lock().unwrap().as_ref().unwrap()), { let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.size.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
                /* for symmetry */
        0
    }

    /// Pos returns the Pos value for the given file offset.
    ///
    /// If offset is negative, the result is the file's start
    /// position; if the offset is too large, the result is
    /// the file's end position (see also go.dev/issue/57490).
    ///
    /// The following invariant, though not true for Pos values
    /// in general, holds for the result p:
    /// f.Pos(f.Offset(p)) == p.
    pub fn pos(&self, offset: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Pos>>> {
        Arc::new(Mutex::new(Some(Pos(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = self.fix_offset(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x + __tmp_y } as i32)))))))
    }

    /// Offset returns the offset for the given file position p.
    ///
    /// If p is before the file's start position (or if p is NoPos),
    /// the result is 0; if p is past the file's end position,
    /// the result is the file size (see also go.dev/issue/57490).
    ///
    /// The following invariant, though not true for offset values
    /// in general, holds for the result offset:
    /// f.Offset(f.Pos(offset)) == offset
    pub fn offset(&self, p: Arc<Mutex<Option<Pos>>>) -> i32 {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.base.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))); self.fix_offset(__method_arg0) }
    }

    /// Line returns the line number for the given file position p;
    /// p must be a [Pos] value in that file or [NoPos].
    pub fn line(&self, p: Arc<Mutex<Option<Pos>>>) -> i32 {
        return (*(*self.position(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).line.lock().unwrap().as_ref().unwrap());
    }

    /// unpack returns the filename and line and column number for a file offset.
    /// If adjusted is set, unpack will return the filename and line information
    /// possibly adjusted by //line comments; otherwise those comments are ignored.
    pub fn unpack(&self, offset: Arc<Mutex<Option<i32>>>, adjusted: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<String>>>, i32, i32) {
    let mut filename: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut column: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        self.mutex.lock();
        { let new_val = { let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *filename.lock().unwrap() = Some(new_val); };
        {
        let mut i = search_ints({ let __field = self.lines.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let __tmp_0 = { let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_1 = { let __tmp_x = { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __seq = { let __seq_holder = self.lines.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; __tmp_x - __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; *line.lock().unwrap() = Some(__tmp_0); *column.lock().unwrap() = Some(__tmp_1); };;
        }
    }
        if { let __v = (*adjusted.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (({ let __len_target = { let __field = self.infos.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // few files have extra line infos
        {
        let mut i = search_line_infos({ let __field = self.infos.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            let mut alt: Option<GoSliceElemPtr<lineInfo>> = Some(GoSliceElemPtr::new(self.infos.clone(), (i) as usize));;
            { let new_val = { let __selector_holder = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *filename.lock().unwrap() = Some(new_val); };;
            {
        let mut i = search_ints({ let __field = self.lines.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }); __tmp_x - __tmp_y })));;
            { let new_val = { let __tmp_x = (*{ let __field = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *line.lock().unwrap() = Some(new_val); };;
            if { let __tmp_x = (*{ let __field = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 0; *column.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = (*{ let __field = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*alt.as_ref().unwrap().borrow().as_ref().unwrap()).offset.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x + __tmp_y }; *column.lock().unwrap() = Some(new_val); };
    };
        }
    };
        }
    }
    }
                // few files have extra line infos
                // i+1 is the line at which the alternative position was recorded
                // line distance from alternative position base
                // alternative column is unknown => relative column is unknown
                // (the current specification for line directives requires
                // this to apply until the next PosBase/line directive,
                // not just until the new newline)
                // the alternative position base is on the current line
                // => column is relative to alternative column
                // TODO(mvdan): move Unlock back under Lock with a defer statement once
                // https://go.dev/issue/38471 is fixed to remove the performance penalty.
        self.mutex.unlock();
        return (filename.clone(), (*line.lock().unwrap().as_ref().unwrap()), (*column.lock().unwrap().as_ref().unwrap()));
    }

    pub fn position_1(&self, p: Arc<Mutex<Option<Pos>>>, adjusted: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Position>>> {
    let mut pos: Arc<Mutex<Option<Position>>> = Arc::new(Mutex::new(Some(Default::default())));

        let mut offset = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.base.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))); self.fix_offset(__method_arg0) };
        { let new_val = offset; *(*pos.lock().unwrap().as_ref().unwrap()).offset.lock().unwrap() = Some(new_val); };
        { let (__tmp_0, __tmp_1, __tmp_2) = self.unpack(Arc::new(Mutex::new(Some(offset))), Arc::new(Mutex::new(Some({ let __arg_holder = adjusted.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*pos.lock().unwrap().as_ref().unwrap()).filename.lock().unwrap() = __moved_tmp_0; *(*pos.lock().unwrap().as_ref().unwrap()).line.lock().unwrap() = Some(__tmp_1); *(*pos.lock().unwrap().as_ref().unwrap()).column.lock().unwrap() = Some(__tmp_2); };
        pos.clone()
    }

    /// PositionFor returns the Position value for the given file position p.
    /// If p is out of bounds, it is adjusted to match the File.Offset behavior.
    /// If adjusted is set, the position may be adjusted by position-altering
    /// //line comments; otherwise those comments are ignored.
    /// p must be a Pos value in f or NoPos.
    pub fn position_for(&self, p: Arc<Mutex<Option<Pos>>>, adjusted: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Position>>> {
    let mut pos: Arc<Mutex<Option<Position>>> = Arc::new(Mutex::new(Some(Default::default())));

        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Pos(Arc::new(Mutex::new(Some(NO_POS as i32)))); __tmp_x != __tmp_y } {
        { let new_val = self.position_1(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = adjusted.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pos.lock().unwrap() = __moved_val; };
    }
        pos.clone()
    }

    /// Position returns the Position value for the given file position p.
    /// If p is out of bounds, it is adjusted to match the File.Offset behavior.
    /// Calling f.Position(p) is equivalent to calling f.PositionFor(p, true).
    pub fn position(&self, p: Arc<Mutex<Option<Pos>>>) -> Arc<Mutex<Option<Position>>> {
    let mut pos: Arc<Mutex<Option<Position>>> = Arc::new(Mutex::new(Some(Default::default())));

        self.position_for(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))))
    }
}

impl FileSet {
    /// Base returns the minimum base offset that must be provided to
    /// [FileSet.AddFile] when adding the next file.
    pub fn base(&self) -> i32 {
        self.mutex.r_lock();
        let mut b = Arc::new(Mutex::new(Some({ let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.mutex.r_unlock();
        return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// AddFile adds a new file with a given filename, base offset, and file size
    /// to the file set s and returns the file. Multiple files may have the same
    /// name. The base offset must not be smaller than the [FileSet.Base], and
    /// size must not be negative. As a special case, if a negative base is provided,
    /// the current value of the [FileSet.Base] is used instead.
    ///
    /// Adding the file will set the file set's [FileSet.Base] value to base + size + 1
    /// as the minimum base value for the next file. The following relationship
    /// exists between a [Pos] value p for a given file offset offs:
    ///
    ///	int(p) = base + offs
    ///
    /// with offs in the range [0, size] and thus p in the range [base, base+size].
    /// For convenience, [File.Pos] may be used to create file-specific position
    /// values from a file offset.
    pub fn add_file(&mut self, filename: Arc<Mutex<Option<String>>>, mut base: Arc<Mutex<Option<i32>>>, size: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<File>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Allocate f outside the critical section.
            let mut f = Arc::new(Mutex::new(Some(File { name: Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), size: Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), lines: Arc::new(Mutex::new(Some(vec![0]))), ..Default::default() })));
            self.mutex.lock();
            let mut s_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        s_defer_captured.mutex.unlock();
    }));
            if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *base.lock().unwrap() = Some(new_val); };
    }
            if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.base.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid base {} (should be >= {})", { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*self.base.lock().unwrap().as_ref().unwrap()))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
            { let new_val = base.lock().unwrap().as_ref().unwrap().clone(); *(*f.lock().unwrap().as_ref().unwrap()).base.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid size {} (should be >= 0)", { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
                        // base >= s.base && size >= 0
            { let __rhs = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = base.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
            if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("token.Pos offset overflow (> 2G of source code in file set)".to_string()) as Box<dyn Any + Send + Sync>);
    }
                        // add the file to the file set
            { let new_val = base.lock().unwrap().as_ref().unwrap().clone(); *self.base.lock().unwrap() = Some(new_val); };
            { let new_val = { let __append_target = self.files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(f.clone()); __append_target.clone() }; self.files = new_val; };
            (*self.last.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(f.clone()));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f.clone();
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                Arc::new(Mutex::new(None))
            }
        }
    }

    /// RemoveFile removes a file from the [FileSet] so that subsequent
    /// queries for its [Pos] interval yield a negative result.
    /// This reduces the memory usage of a long-lived [FileSet] that
    /// encounters an unbounded stream of files.
    ///
    /// Removing a file that does not belong to the set has no effect.
    pub fn remove_file(&mut self, file: Arc<Mutex<Option<File>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (*self.last.lock().unwrap().as_mut().unwrap()).compare_and_swap(sync_atomic::GoPtr::local(file.clone()), sync_atomic::GoPtr::nil());
            self.mutex.lock();
            let mut s_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        s_defer_captured.mutex.unlock();
    }));
            {
        let mut i = search_files({ let __field = self.files.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*file.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __left = { let __seq = { let __seq_holder = self.files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __right = file.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
            let mut last: Option<GoSliceElemPtr<Arc<Mutex<Option<File>>>>> = Some(GoSliceElemPtr::new(self.files.clone(), ({ let __tmp_x = (({ let __len_target = { let __field = self.files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize));;
            { let new_val = slices::delete::<Vec<Arc<Mutex<Option<File>>>>, File>({ let __field = self.files.clone(); __field }, Arc::new(Mutex::new(Some(i))), Arc::new(Mutex::new(Some({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y })))); self.files = new_val; };;
            { let new_val = Arc::new(Mutex::new(None)); *last.as_ref().unwrap().borrow_mut() = Some(new_val); };;
        }
    }

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                ()
            }
        }
    }

    /// Iterate calls f for the files in the file set in the order they were added
    /// until f returns false.
    pub fn iterate(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<File>>>) -> bool + Send + Sync>>>>) {
        let mut i = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut file: Arc<Mutex<Option<File>>> = Arc::new(Mutex::new(None));
        self.mutex.r_lock();
        if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = self.files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(); file = new_val; };
    }
        self.mutex.r_unlock();
        if (*file.lock().unwrap()).is_none() || !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<File>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<File>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(file.clone()) } {
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    pub fn file_1(&self, p: Arc<Mutex<Option<Pos>>>) -> GoPtr<File> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // common case: p is in last file.
            {
        let mut f: GoPtr<File> = { let __go_ptr = (*self.last.lock().unwrap().as_mut().unwrap()).load().clone(); match __go_ptr { sync_atomic::GoPtr::Nil => GoPtr::nil(), sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()), sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr), sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };;
        if !f.is_nil() && { let __tmp_x = (*{ let __ptr_value = f.borrow(); __ptr_value.as_ref().unwrap().base.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*{ let __ptr_value = f.borrow(); __ptr_value.as_ref().unwrap().base.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = f.borrow(); __ptr_value.as_ref().unwrap().size.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return f.clone();
    };
        }
    }
            self.mutex.r_lock();
            let mut s_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        s_defer_captured.mutex.r_unlock();
    }));
                        // p is not in last file - search all files
            {
        let mut i = search_files({ let __field = self.files.clone(); __field }, Arc::new(Mutex::new(Some((*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            let mut f = { let __seq = { let __seq_holder = self.files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();;
            if { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).base.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        (*self.last.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(f.clone()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return GoPtr::local(f.clone());
    }
    };
        }
    }
                        // f.base <= int(p) by definition of searchFiles
                        // Update cache of last file. A race is ok,
                        // but an exclusive lock causes heavy contention.
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return GoPtr::nil();
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                GoPtr::nil()
            }
        }
    }

    /// File returns the file that contains the position p.
    /// If no such file is found (for instance for p == [NoPos]),
    /// the result is nil.
    pub fn file(&self, p: Arc<Mutex<Option<Pos>>>) -> GoPtr<File> {
    let mut f: GoPtr<File> = GoPtr::nil();

        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Pos(Arc::new(Mutex::new(Some(NO_POS as i32)))); __tmp_x != __tmp_y } {
        f = self.file_1(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        f.clone()
    }

    /// PositionFor converts a [Pos] p in the fileset into a [Position] value.
    /// If adjusted is set, the position may be adjusted by position-altering
    /// //line comments; otherwise those comments are ignored.
    /// p must be a [Pos] value in s or [NoPos].
    pub fn position_for(&self, p: Arc<Mutex<Option<Pos>>>, adjusted: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Position>>> {
    let mut pos: Arc<Mutex<Option<Position>>> = Arc::new(Mutex::new(Some(Default::default())));

        if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Pos(Arc::new(Mutex::new(Some(NO_POS as i32)))); __tmp_x != __tmp_y } {
        {
        let mut f: GoPtr<File> = self.file_1(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if !f.is_nil() {
            return { let __recv_value = f.borrow(); let __result = (*__recv_value.as_ref().unwrap()).position_1(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = adjusted.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        }
    }
    }
        pos.clone()
    }

    /// Position converts a [Pos] p in the fileset into a Position value.
    /// Calling s.Position(p) is equivalent to calling s.PositionFor(p, true).
    pub fn position(&self, p: Arc<Mutex<Option<Pos>>>) -> Arc<Mutex<Option<Position>>> {
    let mut pos: Arc<Mutex<Option<Position>>> = Arc::new(Mutex::new(Some(Default::default())));

        self.position_for(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))))
    }
}

pub fn search_line_infos(a: Arc<Mutex<Option<Vec<lineInfo>>>>, x: Arc<Mutex<Option<i32>>>) -> i32 {
    let (mut i, mut found) = slices::binary_search_func::<Vec<lineInfo>, lineInfo, i32>({ let __slice_holder = a.clone(); { let __slice_guard = __slice_holder.lock().unwrap(); Arc::new(Mutex::new(__slice_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Arc::new(Mutex::new(Some(__elem)))).collect::<Vec<_>>()))) } }, Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<lineInfo>>>, x: Arc<Mutex<Option<i32>>>| -> i32 {
        cmp::compare::<i32>({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*x.lock().unwrap().as_ref().unwrap()).clone())
    }) as Box<dyn FnMut(Arc<Mutex<Option<lineInfo>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>))));
    if !found {
                // We want the lineInfo containing x, but if we didn't
                // find x then i is the next one.
        { i -= 1; }
    }
        // We want the lineInfo containing x, but if we didn't
        // find x then i is the next one.
    i
}

/// NewFileSet creates a new file set.
pub fn new_file_set() -> Arc<Mutex<Option<FileSet>>> {
    Arc::new(Mutex::new(Some(FileSet { base: Arc::new(Mutex::new(Some(1))), ..Default::default() })))
}

pub fn search_files(a: Arc<Mutex<Option<Vec<Arc<Mutex<Option<File>>>>>>>, x: Arc<Mutex<Option<i32>>>) -> i32 {
    let (mut i, mut found) = slices::binary_search_func::<Vec<Arc<Mutex<Option<File>>>>, File, i32>(a.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<File>>>, x: Arc<Mutex<Option<i32>>>| -> i32 {
        cmp::compare::<i32>({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*x.lock().unwrap().as_ref().unwrap()).clone())
    }) as Box<dyn FnMut(Arc<Mutex<Option<File>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>))));
    if !found {
                // We want the File containing x, but if we didn't
                // find x then i is the next one.
        { i -= 1; }
    }
        // We want the File containing x, but if we didn't
        // find x then i is the next one.
    i
}

pub fn search_ints(a: Arc<Mutex<Option<Vec<i32>>>>, x: Arc<Mutex<Option<i32>>>) -> i32 {
        // This function body is a manually inlined version of:
        //
        //   return sort.Search(len(a), func(i int) bool { return a[i] > x }) - 1
        //
        // With better compiler optimizations, this may not be needed in the
        // future, but at the moment this change improves the go/printer
        // benchmark performance by ~30%. This has a direct impact on the
        // speed of gofmt and thus seems worthwhile (2011-04-29).
        // TODO(gri): Remove this when compilers have caught up.
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some((*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut h = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));

                // i ≤ h < j
        if { let __tmp_x = { let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
    }
        // avoid overflow when computing h
        // i ≤ h < j
    return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y };
}

impl GoValueClone for Position {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for File {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for lineInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FileSet {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
