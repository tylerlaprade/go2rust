use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
};

use crate::{map_swiss::{SwissMapType}};

use std::sync::{Arc, Mutex};

/// Select the map type that this binary is built using. This is for common
/// lookup methods like Type.Key to know which type to use.
///
/// Note that mapType *must not be used by any functions called in the
/// compiler to build a target program* because the compiler must use the map
/// type determined by run-time GOEXPERIMENT, not the build tags used to build
/// the compiler.
///
/// TODO(prattmic): This package is rather confusing because it has many
/// functions that can't be used by the compiler (e.g., Type.Uncommon depends on
/// the layout of type + uncommon objects in the binary. It would be incorrect
/// for an ad-hoc local Type object). It may be best to move code that isn't
/// usable by the compiler out of the package.
pub type mapType = Arc<Mutex<Option<SwissMapType>>>;
