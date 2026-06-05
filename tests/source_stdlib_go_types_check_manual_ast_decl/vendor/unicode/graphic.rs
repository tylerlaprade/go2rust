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


/// IsGraphic reports whether the rune is defined as a Graphic by Unicode.
/// Such characters include letters, marks, numbers, punctuation, symbols, and
/// spaces, from categories [L], [M], [N], [P], [S], [Zs].
pub fn is_graphic(r: Arc<Mutex<Option<i32>>>) -> bool {
        // We convert to uint32 to avoid the extra test for negative,
        // and in the index we convert to uint8 to avoid the range check.
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_LATIN1 as u32; __tmp_x <= __tmp_y } {
        return { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = properties.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()) as usize].clone() }; let __tmp_y = PG as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }
    r#in(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GraphicRanges.clone())
}

/// In reports whether the rune is a member of one of the ranges.
pub fn r#in(r: Arc<Mutex<Option<i32>>>, ranges: Arc<Mutex<Option<Vec<Arc<Mutex<Option<RangeTable>>>>>>>) -> bool {
    { let __range_holder = ranges.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for inside in __range_values.iter() {
        if is((*inside).clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
    } }
    false
}

/// IsLetter reports whether the rune is a letter (category [L]).
pub fn is_letter(r: Arc<Mutex<Option<i32>>>) -> bool {
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_LATIN1 as u32; __tmp_x <= __tmp_y } {
        return { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = properties.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()) as usize].clone() }; let __tmp_y = (P_LMASK) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }
    is_excluding_latin({ let __arg_holder = Letter.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// IsSpace reports whether the rune is a space character as defined
/// by Unicode's White Space property; in the Latin-1 space
/// this is
///
///	'\t', '\n', '\v', '\f', '\r', ' ', U+0085 (NEL), U+00A0 (NBSP).
///
/// Other definitions of spacing characters are set by category
/// Z and property [Pattern_White_Space].
pub fn is_space(r: Arc<Mutex<Option<i32>>>) -> bool {
        // This property isn't the same as Z; special-case it.
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_LATIN1 as u32; __tmp_x <= __tmp_y } {
        { let _switch_val = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('\t' as i32)) || _switch_val == (('\n' as i32)) || _switch_val == (('\u{b}' as i32)) || _switch_val == (('\u{c}' as i32)) || _switch_val == (('\r' as i32)) || _switch_val == ((' ' as i32)) || _switch_val == (0x85 as i32) || _switch_val == (0xA0 as i32) {
            return true;
        }
    }
        return false;
    }
    is_excluding_latin({ let __arg_holder = White_Space.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
