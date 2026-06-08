use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{os_darwin::{__S_I__U_S_E_R}, signal_darwin_arm64::{sigctxt}};

impl crate::signal_darwin_arm64::sigctxt {
    /// sigFromUser reports whether the signal was sent because of a call
    /// to kill.
    ///
    ///go:nosplit
    pub fn sig_from_user(&self) -> bool {
        return { let __tmp_x = self.sigcode(); let __tmp_y = __S_I__U_S_E_R as u64; __tmp_x == __tmp_y };
    }

    /// sigFromSeccomp reports whether the signal was sent from seccomp.
    ///
    ///go:nosplit
    pub fn sig_from_seccomp(&self) -> bool {
        false
    }
}