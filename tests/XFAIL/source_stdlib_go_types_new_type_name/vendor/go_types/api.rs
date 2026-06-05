use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_register_embedded_owner, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
use crate::builtins::*;
use crate::call::*;
use crate::chan::*;
use crate::check::*;
use crate::r#const::*;
use crate::context::*;
use crate::conversions::*;
use crate::decl::*;
use crate::errors::*;
use crate::errsupport::*;
use crate::eval::*;
use crate::expr::*;
use crate::exprstring::*;
use crate::format::*;
use crate::gccgosizes::*;
use crate::gcsizes::*;
use crate::index::*;
use crate::infer::*;
use crate::initorder::*;
use crate::instantiate::*;
use crate::interface::*;
use crate::iter::*;
use crate::labels::*;
use crate::literals::*;
use crate::lookup::*;
use crate::map::*;
use crate::methodset::*;
use crate::mono::*;
use crate::named::*;
use crate::object::*;
use crate::objset::*;
use crate::operand::*;
use crate::package::*;
use crate::pointer::*;
use crate::predicates::*;
use crate::recording::*;
use crate::resolver::*;
use crate::r#return::*;
use crate::scope::*;
use crate::scope2::*;
use crate::selection::*;
use crate::signature::*;
use crate::sizes::*;
use crate::slice::*;
use crate::stmt::*;
use crate::r#struct::*;
use crate::subst::*;
use crate::termlist::*;
use crate::tuple::*;
use crate::r#type::*;
use crate::typelists::*;
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use internal_types_errors::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// An Error describes a type-checking error; it implements the error interface.
/// A "soft" error is an error that still permits a valid interpretation of a
/// package (such as "unused variable"); "hard" errors may lead to unpredictable
/// behavior if ignored.
#[derive(Clone)]
pub struct Error {
    pub fset: Arc<Mutex<Option<go_token::position::FileSet>>>,
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub msg: Arc<Mutex<Option<String>>>,
    pub soft: Arc<Mutex<Option<bool>>>,
    pub go116code: Arc<Mutex<Option<Code>>>,
    pub go116start: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub go116end: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl Error {
    pub fn __go_value_clone(&self) -> Self {
        Self { fset: self.fset.clone(), pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, msg: { let __guard = self.msg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, soft: { let __guard = self.soft.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go116code: { let __guard = self.go116code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go116start: { let __guard = self.go116start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go116end: { let __guard = self.go116end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Error {
    fn default() -> Self {
        Self { fset: Arc::new(Mutex::new(None)), pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), msg: Arc::new(Mutex::new(Some(String::new()))), soft: Arc::new(Mutex::new(Some(false))), go116code: Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0))))))), go116start: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), go116end: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl GoJsonDecode for Error {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Msg") {
            out.msg = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Soft") {
            out.soft = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// An Importer resolves import paths to Packages.
///
/// CAUTION: This interface does not support the import of locally
/// vendored packages. See https://golang.org/s/go15vendor.
/// If possible, external implementations should implement [ImporterFrom].
pub trait Importer: std::fmt::Display + Any {
    fn __go_clone_box_importer(&self) -> Box<dyn Importer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_importer(&self, other: &(dyn Importer + Send + Sync)) -> bool;
    fn import(&self, path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn Importer + Send + Sync> {
    fn clone(&self) -> Self {
        Importer::__go_clone_box_importer(self.as_ref())
    }
}

/// ImportMode is reserved for future use.
#[derive(Debug, Clone, Default)]
pub struct ImportMode(pub Arc<Mutex<Option<i32>>>);

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

impl PartialEq<i32> for ImportMode {
    fn eq(&self, other: &i32) -> bool {
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

impl PartialOrd<i32> for ImportMode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ImportMode> for i32 {
    fn eq(&self, other: &ImportMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ImportMode> for i32 {
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

impl std::ops::Add<i32> for ImportMode {
    type Output = ImportMode;
    fn add(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ImportMode> for i32 {
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

impl std::ops::Sub<i32> for ImportMode {
    type Output = ImportMode;
    fn sub(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ImportMode> for i32 {
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

impl std::ops::Mul<i32> for ImportMode {
    type Output = ImportMode;
    fn mul(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ImportMode> for i32 {
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

impl std::ops::Div<i32> for ImportMode {
    type Output = ImportMode;
    fn div(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ImportMode> for i32 {
    type Output = ImportMode;
    fn div(self, other: ImportMode) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ImportMode {
    type Output = ImportMode;
    fn neg(self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ImportMode {
    type Output = ImportMode;
    fn rem(self, other: Self) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for ImportMode {
    type Output = ImportMode;
    fn rem(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ImportMode> for i32 {
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

impl std::ops::BitAnd<i32> for ImportMode {
    type Output = ImportMode;
    fn bitand(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ImportMode> for i32 {
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

impl std::ops::BitOr<i32> for ImportMode {
    type Output = ImportMode;
    fn bitor(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ImportMode> for i32 {
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

impl std::ops::BitXor<i32> for ImportMode {
    type Output = ImportMode;
    fn bitxor(self, other: i32) -> ImportMode {
        ImportMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ImportMode> for i32 {
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


/// An ImporterFrom resolves import paths to packages; it
/// supports vendoring per https://golang.org/s/go15vendor.
/// Use go/importer to obtain an ImporterFrom implementation.
pub trait ImporterFrom: Importer + std::fmt::Display + Any {
    fn __go_clone_box_importer_from(&self) -> Box<dyn ImporterFrom + Send + Sync>;
    fn __go_eq_importer_from(&self, other: &(dyn ImporterFrom + Send + Sync)) -> bool;
    fn import_from(&self, path: Arc<Mutex<Option<String>>>, dir: Arc<Mutex<Option<String>>>, mode: Arc<Mutex<Option<ImportMode>>>) -> (Arc<Mutex<Option<Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn ImporterFrom + Send + Sync> {
    fn clone(&self) -> Self {
        ImporterFrom::__go_clone_box_importer_from(self.as_ref())
    }
}

impl Importer for Box<dyn ImporterFrom + Send + Sync> {
    fn __go_clone_box_importer(&self) -> Box<dyn Importer + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Importer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_importer(&self, other: &(dyn Importer + Send + Sync)) -> bool {
        (**self).__go_eq_importer(other)
    }
    fn import(&self, path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::package::Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (**self).import(path)
    }
}

/// A Config specifies the configuration for type checking.
/// The zero value for Config is a ready-to-use default configuration.
#[derive(Clone)]
pub struct Config {
    pub context: Arc<Mutex<Option<Context>>>,
    pub go_version: Arc<Mutex<Option<String>>>,
    pub ignore_func_bodies: Arc<Mutex<Option<bool>>>,
    pub fake_import_c: Arc<Mutex<Option<bool>>>,
    pub go115_uses_cgo: Arc<Mutex<Option<bool>>>,
    pub __trace: Arc<Mutex<Option<bool>>>,
    pub error: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync>>>>,
    pub importer: Arc<Mutex<Option<Box<dyn Importer + Send + Sync>>>>,
    pub sizes: Arc<Mutex<Option<Box<dyn Sizes + Send + Sync>>>>,
    pub disable_unused_import_check: Arc<Mutex<Option<bool>>>,
    pub __error_u_r_l: Arc<Mutex<Option<String>>>,
    pub __enable_alias: Arc<Mutex<Option<bool>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        Self { context: self.context.clone(), go_version: { let __guard = self.go_version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ignore_func_bodies: { let __guard = self.ignore_func_bodies.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fake_import_c: { let __guard = self.fake_import_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go115_uses_cgo: { let __guard = self.go115_uses_cgo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __trace: { let __guard = self.__trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, error: self.error.clone(), importer: self.importer.clone(), sizes: self.sizes.clone(), disable_unused_import_check: { let __guard = self.disable_unused_import_check.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __error_u_r_l: { let __guard = self.__error_u_r_l.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __enable_alias: { let __guard = self.__enable_alias.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Config {
    fn default() -> Self {
        Self { context: Arc::new(Mutex::new(None)), go_version: Arc::new(Mutex::new(Some(String::new()))), ignore_func_bodies: Arc::new(Mutex::new(Some(false))), fake_import_c: Arc::new(Mutex::new(Some(false))), go115_uses_cgo: Arc::new(Mutex::new(Some(false))), __trace: Arc::new(Mutex::new(Some(false))), error: Arc::new(Mutex::new(None)), importer: Arc::new(Mutex::new(None)), sizes: Arc::new(Mutex::new(None)), disable_unused_import_check: Arc::new(Mutex::new(Some(false))), __error_u_r_l: Arc::new(Mutex::new(Some(String::new()))), __enable_alias: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.context.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.go_version.lock().unwrap().as_ref().unwrap()), (*self.ignore_func_bodies.lock().unwrap().as_ref().unwrap()), (*self.fake_import_c.lock().unwrap().as_ref().unwrap()), (*self.go115_uses_cgo.lock().unwrap().as_ref().unwrap()), (*self.__trace.lock().unwrap().as_ref().unwrap()), "<func>", (*self.importer.lock().unwrap().as_ref().unwrap()), (*self.sizes.lock().unwrap().as_ref().unwrap()), (*self.disable_unused_import_check.lock().unwrap().as_ref().unwrap()), (*self.__error_u_r_l.lock().unwrap().as_ref().unwrap()), (*self.__enable_alias.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Config {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("GoVersion") {
            out.go_version = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IgnoreFuncBodies") {
            out.ignore_func_bodies = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("FakeImportC") {
            out.fake_import_c = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("DisableUnusedImportCheck") {
            out.disable_unused_import_check = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// Info holds result type information for a type-checked package.
/// Only the information for which a map is provided is collected.
/// If the package has type errors, the collected information may
/// be incomplete.
#[derive(Clone, Default)]
pub struct Info {
    pub types: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn go_ast::r#mod::Expr + Send + Sync>>, Arc<Mutex<Option<TypeAndValue>>>>>>>,
    pub instances: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<go_ast::r#mod::Ident>, Arc<Mutex<Option<Instance>>>>>>>,
    pub defs: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<go_ast::r#mod::Ident>, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>,
    pub uses: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<go_ast::r#mod::Ident>, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>,
    pub implicits: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn go_ast::r#mod::Node + Send + Sync>>, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>,
    pub selections: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<go_ast::r#mod::SelectorExpr>, Arc<Mutex<Option<Selection>>>>>>>,
    pub scopes: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn go_ast::r#mod::Node + Send + Sync>>, Arc<Mutex<Option<Scope>>>>>>>,
    pub init_order: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Initializer>>>>>>>,
    pub file_versions: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<go_ast::r#mod::File>, Arc<Mutex<Option<String>>>>>>>,
}

impl Info {
    pub fn __go_value_clone(&self) -> Self {
        Self { types: self.types.clone(), instances: self.instances.clone(), defs: self.defs.clone(), uses: self.uses.clone(), implicits: self.implicits.clone(), selections: self.selections.clone(), scopes: self.scopes.clone(), init_order: self.init_order.clone(), file_versions: self.file_versions.clone() }
    }
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", format_map(&self.types), format_map(&self.instances), format_map(&self.defs), format_map(&self.uses), format_map(&self.implicits), format_map(&self.selections), format_map(&self.scopes), format_slice_wrapped(&self.init_order), format_map(&self.file_versions))
    }
}

impl GoJsonDecode for Info {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// TypeAndValue reports the type and value (for constants)
/// of the corresponding expression.
#[derive(Clone)]
pub struct TypeAndValue {
    pub mode: Arc<Mutex<Option<operandMode>>>,
    pub r#type: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub value: Arc<Mutex<Option<constant_Value>>>,
}

impl TypeAndValue {
    pub fn __go_value_clone(&self) -> Self {
        Self { mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: self.r#type.clone(), value: self.value.clone() }
    }
}


impl Default for TypeAndValue {
    fn default() -> Self {
        Self { mode: Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(0))))))), r#type: Arc::new(Mutex::new(None)), value: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for TypeAndValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.mode.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for TypeAndValue {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Instance reports the type arguments and instantiated type for type and
/// function instantiations. For type instantiations, [Type] will be of dynamic
/// type *[Named]. For function instantiations, [Type] will be of dynamic type
/// *Signature.
#[derive(Clone, Default)]
pub struct Instance {
    pub type_args: Arc<Mutex<Option<TypeList>>>,
    pub r#type: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl Instance {
    pub fn __go_value_clone(&self) -> Self {
        Self { type_args: self.type_args.clone(), r#type: self.r#type.clone() }
    }
}

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.type_args.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.r#type.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Instance {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An Initializer describes a package-level variable, or a list of variables in case
/// of a multi-valued initialization expression, and the corresponding initialization
/// expression.
#[derive(Clone, Default)]
pub struct Initializer {
    pub lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>,
    pub rhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
}

impl Initializer {
    pub fn __go_value_clone(&self) -> Self {
        Self { lhs: self.lhs.clone(), rhs: self.rhs.clone() }
    }
}

impl std::fmt::Display for Initializer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Initializer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Error {
    /// Error returns an error string formatted as follows:
    /// filename:line:column: message
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{}: {}", (*(*self.fset.lock().unwrap().as_ref().unwrap()).position({ let __field = self.pos.clone(); __field }).lock().unwrap().as_ref().unwrap()), (*self.msg.lock().unwrap().as_ref().unwrap())))))
    }
}

impl StdError for Error {}


impl Info {
    pub fn record_types(&self) -> bool {
        return { let __nil_target = self.types.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
    }

    /// TypeOf returns the type of expression e, or nil if not found.
    /// Precondition: the Types, Uses and Defs maps are populated.
    pub fn type_of(&self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        {
        let (mut t, mut ok) = { let __map = { let __map_holder = self.types.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(e.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };;
        if ok {
            return (*t.lock().unwrap().as_ref().unwrap()).r#type.clone();;
        }
    }
        {
        let (mut id, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if (*id.lock().unwrap()).is_some() {
            {
        let mut obj = self.object_of(id.clone());;
        if (*obj.lock().unwrap()).is_some() {
            return (*obj.lock().unwrap().as_ref().unwrap()).r#type().clone();;
        }
    };
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// ObjectOf returns the object denoted by the specified id,
    /// or nil if not found.
    ///
    /// If id is an embedded struct field, [Info.ObjectOf] returns the field (*[Var])
    /// it defines, not the type (*[TypeName]) it uses.
    ///
    /// Precondition: the Uses and Defs maps are populated.
    pub fn object_of(&self, id: Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        {
        let mut obj = { let __map = { let __map_holder = self.defs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(id.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*obj.lock().unwrap()).is_some() {
            return obj.clone();;
        }
    }
        { let __map = { let __map_holder = self.uses.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(id.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }
    }

    /// PkgNameOf returns the local package name defined by the import,
    /// or nil if not found.
    ///
    /// For dot-imports, the package name is ".".
    ///
    /// Precondition: the Defs and Implicts maps are populated.
    pub fn pkg_name_of(&self, imp: Arc<Mutex<Option<go_ast::r#mod::ImportSpec>>>) -> Arc<Mutex<Option<crate::object::PkgName>>> {
        let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __nil_target = (*imp.lock().unwrap().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __iface_handle = { let __map = { let __map_holder = self.defs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new((*imp.lock().unwrap().as_ref().unwrap()).name.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *obj.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let __iface_handle = { let __map = { let __map_holder = self.implicits.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ImportSpecPtr(imp.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *obj.lock().unwrap() = (*__iface_guard).clone(); };
    }
        let (mut pkgname, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::PkgNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::PkgName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::PkgName>)), false)
        }
    });
        return pkgname.clone();
    }
}

impl TypeAndValue {
    /// IsVoid reports whether the corresponding expression
    /// is a function call without results.
    pub fn is_void(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); __tmp_x == __tmp_y };
    }

    /// IsType reports whether the corresponding expression specifies a type.
    pub fn is_type(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8)))); __tmp_x == __tmp_y };
    }

    /// IsBuiltin reports whether the corresponding expression denotes
    /// a (possibly parenthesized) built-in function.
    pub fn is_builtin(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8)))); __tmp_x == __tmp_y };
    }

    /// IsValue reports whether the corresponding expression is a value.
    /// Builtins are not considered values. Constant values have a non-
    /// nil Value.
    pub fn is_value(&self) -> bool {
        { let _switch_val = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(COMMAOK as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(COMMAERR as u8))))) {
            return true;
        }
    }
        false
    }

    /// IsNil reports whether the corresponding expression denotes the
    /// predeclared value nil.
    pub fn is_nil(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); __tmp_x == __tmp_y } && { let __left_holder = self.r#type.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq };
    }

    /// Addressable reports whether the corresponding expression
    /// is addressable (https://golang.org/ref/spec#Address_operators).
    pub fn addressable(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x == __tmp_y };
    }

    /// Assignable reports whether the corresponding expression
    /// is assignable to (provided a value of the right type).
    pub fn assignable(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8)))); __tmp_x == __tmp_y };
    }

    /// HasOk reports whether the corresponding expression may be
    /// used on the rhs of a comma-ok assignment.
    pub fn has_ok(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(COMMAOK as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8)))); __tmp_x == __tmp_y };
    }
}

impl Initializer {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let __range_holder = self.lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(", ".to_string());
    }
        (*buf.lock().unwrap().as_mut().unwrap()).write_string({ let __recv = lhs.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.name(); __result });
    } }
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(" = ".to_string());
        write_expr(buf.clone(), self.rhs.clone());
        return (*buf.lock().unwrap().as_mut().unwrap()).string();
    }
}

impl Config {
    /// Check type-checks a package and returns the resulting package object and
    /// the first error if any. Additionally, if info != nil, Check populates each
    /// of the non-nil maps in the [Info] struct.
    ///
    /// The package is marked as complete if no errors occurred, otherwise it is
    /// incomplete. See [Config.Error] for controlling behavior in the presence of
    /// errors.
    ///
    /// The package is specified by a list of *ast.Files and corresponding
    /// file set, and the package path the package is identified with.
    /// The clean path must not be empty or dot (".").
    pub fn check(&self, path: Arc<Mutex<Option<String>>>, fset: Arc<Mutex<Option<go_token::position::FileSet>>>, files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::File>>>>>>>, info: Arc<Mutex<Option<Info>>>) -> (Arc<Mutex<Option<crate::package::Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut pkg = new_package(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string()))));
        return (pkg.clone(), { let __recv = new_checker(Arc::new(Mutex::new(Some(self.clone()))), fset.clone(), pkg.clone(), info.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).files(files.clone()); __result });
    }
}

impl GoValueClone for Error {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Config {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Info {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeAndValue {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Instance {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Initializer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
