use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped, go_any_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::hashtriemap::*;
use crate::mutex::*;
use crate::once::*;
use crate::oncefunc::*;
use crate::pool::*;
use crate::poolqueue::*;
use crate::runtime::*;
use crate::runtime2::*;
use crate::rwmutex::*;
use crate::waitgroup::*;

use std::any::Any;
use std::fmt::{Display, Formatter};

/// noCopy may be added to structs which must not be copied
/// after the first use.
///
/// See https://golang.org/issues/8005#issuecomment-190753527
/// for details.
///
/// Note that it must not be embedded, due to the Lock and Unlock methods.
#[derive(Debug, Clone, Default)]
pub struct noCopy {
}

impl noCopy {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for noCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for noCopy {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl noCopy {
    /// Lock is a no-op used by -copylocks checker from `go vet`.
    pub fn lock(&self) {
    }

    pub fn unlock(&self) {
    }
}

impl GoValueClone for noCopy {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
