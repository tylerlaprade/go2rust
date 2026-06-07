use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alias::*;
use crate::api::*;
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

use std::sync::{Arc, Mutex};

/// Identical reports whether x and y are identical types.
/// Receivers of [Signature] types are ignored.
///
/// Predicates such as [Identical], [Implements], and
/// [Satisfies] assume that both operands belong to a
/// consistent collection of symbols ([Object] values).
/// For example, two [Named] types can be identical only if their
/// [Named.Obj] methods return the same [TypeName] symbol.
/// A collection of symbols is consistent if, for each logical
/// package whose path is P, the creation of those symbols
/// involved at most one call to [NewPackage](P, ...).
/// To ensure consistency, use a single [Importer] for
/// all loaded packages and their dependencies.
/// For more information, see https://github.com/golang/go/issues/57497.
pub fn identical(x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let mut c: Arc<Mutex<Option<comparer>>> = Arc::new(Mutex::new(Some(Default::default())));
    return (*c.lock().unwrap().as_ref().unwrap()).identical(x.clone(), y.clone(), Arc::new(Mutex::new(None)));
}

/// IdenticalIgnoreTags reports whether x and y are identical types if tags are ignored.
/// Receivers of [Signature] types are ignored.
pub fn identical_ignore_tags(x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let mut c: Arc<Mutex<Option<comparer>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = true; *(*c.lock().unwrap().as_ref().unwrap()).ignore_tags.lock().unwrap() = Some(new_val); };
    return (*c.lock().unwrap().as_ref().unwrap()).identical(x.clone(), y.clone(), Arc::new(Mutex::new(None)));
}