use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::casetables::*;
use crate::digit::*;
use crate::letter::*;
use crate::tables::*;

use std::sync::{Arc, Mutex};

pub(crate) const P_C: i32 = 1 << 0;
pub(crate) const P_P: i32 = 1 << 1;
pub(crate) const P_N: i32 = 1 << 2;
pub(crate) const P_S: i32 = 1 << 3;
pub(crate) const P_Z: i32 = 1 << 4;
pub(crate) const P_LU: i32 = 1 << 5;
pub(crate) const P_LL: i32 = 1 << 6;
pub(crate) const PP: i32 = 1 << 7;
pub(crate) const PG: i32 = PP | P_Z;
pub(crate) const P_LO: i32 = P_LL | P_LU;
pub(crate) const P_LMASK: i32 = P_LO;


pub static GraphicRanges: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<crate::letter::RangeTable>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static PrintRanges: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<crate::letter::RangeTable>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *GraphicRanges.lock().unwrap() = Some(vec![]);
    *PrintRanges.lock().unwrap() = Some(vec![]);
    *PrintRanges.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![{ let __arg_holder = L.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = M.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = N.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = P.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = S.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }]))).lock().unwrap().as_ref().unwrap()).clone());
    *GraphicRanges.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![{ let __arg_holder = L.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = M.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = N.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = P.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = S.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = Zs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *GraphicRanges.lock().unwrap() = Some(vec![]);
    *PrintRanges.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_79() {
    *PrintRanges.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![{ let __arg_holder = L.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = M.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = N.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = P.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = S.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_87() {
    *GraphicRanges.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![{ let __arg_holder = L.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = M.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = N.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = P.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = S.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __arg_holder = Zs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }]))).lock().unwrap().as_ref().unwrap()).clone());
}


/// IsLetter reports whether the rune is a letter (category [L]).
pub fn is_letter(r: Arc<Mutex<Option<i32>>>) -> bool {
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_LATIN1 as u32; __tmp_x <= __tmp_y } {
        return { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = properties.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()) as usize].clone() }; let __tmp_y = (P_LMASK) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }
    is_excluding_latin({ let __arg_holder = Letter.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
