use go2rust_stdlib_stubs::*;

use crate::{GoByteSequence};

use crate::compare_native::*;
use crate::count_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_arm64::*;
use crate::index_native::*;
use crate::indexbyte_native::*;
use crate::lastindexbyte_generic::*;

use std::sync::{Arc, Mutex};

pub(crate) const OFFSET_X86_HAS_S_S_E42: usize = std::mem::offset_of!(internal_cpu::X86, has_s_s_e42);
pub(crate) const OFFSET_X86_HAS_A_V_X2: usize = std::mem::offset_of!(internal_cpu::X86, has_a_v_x2);
pub(crate) const OFFSET_X86_HAS_P_O_P_C_N_T: usize = std::mem::offset_of!(internal_cpu::X86, has_p_o_p_c_n_t);
pub(crate) const OFFSET_S390X_HAS_V_X: usize = std::mem::offset_of!(internal_cpu::S390X, has_v_x);
pub(crate) const OFFSET_P_P_C64_HAS_P_O_W_E_R9: usize = std::mem::offset_of!(internal_cpu::PPC64, is_p_o_w_e_r9);


pub const PRIME_R_K: i32 = 16777619;


pub static MaxLen: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *MaxLen.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *MaxLen.lock().unwrap() = Some(0);
}


/// HashStr returns the hash and the appropriate multiplicative
/// factor for use in Rabin-Karp algorithm.
pub fn hash_str<T: GoByteSequence + Clone + Send + Sync + 'static>(sep: Arc<Mutex<Option<T>>>) -> (u32, u32) {
    let mut hash = Arc::new(Mutex::new(Some(0 as u32)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*sep.lock().unwrap().as_ref().unwrap()).go_len() as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*hash.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PRIME_R_K as u32; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*sep.lock().unwrap().as_ref().unwrap()).go_byte(({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *hash.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut pow: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(1)));let mut sq: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(PRIME_R_K as u32)));
    let mut i = Arc::new(Mutex::new(Some((*sep.lock().unwrap().as_ref().unwrap()).go_len() as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let __rhs = (*sq.lock().unwrap().as_ref().unwrap()); let mut guard = pow.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }
        { let __rhs = (*sq.lock().unwrap().as_ref().unwrap()); let mut guard = sq.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let __rhs = 1; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
    return ({ let __v = (*hash.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pow.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// IndexRabinKarp uses the Rabin-Karp search algorithm to return the index of the
/// first occurrence of sep in s, or -1 if not present.
pub fn index_rabin_karp<T: GoByteSequence + Clone + Send + Sync + 'static>(s: Arc<Mutex<Option<T>>>, sep: Arc<Mutex<Option<T>>>) -> i32 {
        // Rabin-Karp search
    let (mut hashss, mut pow) = hash_str::<T>(sep.clone());
    let mut n = Arc::new(Mutex::new(Some((*sep.lock().unwrap().as_ref().unwrap()).go_len() as i32)));
    let mut h: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PRIME_R_K as u32; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_byte(({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *h.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = hashss; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string(0, Some(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))))).lock().unwrap().as_ref().unwrap()).go_to_string()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*Arc::new(Mutex::new(Some((*sep.lock().unwrap().as_ref().unwrap()).go_to_string()))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return 0;
    }
    let mut i = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); __tmp_x < __tmp_y } {
        { let __rhs = PRIME_R_K as u32; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let __rhs = (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_byte(({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) as u32))).lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = { let __tmp_x = pow; let __tmp_y = (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_byte(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = hashss; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize, Some(({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))))).lock().unwrap().as_ref().unwrap()).go_to_string()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*Arc::new(Mutex::new(Some((*sep.lock().unwrap().as_ref().unwrap()).go_to_string()))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };
    }
    }
    -(1)
}

/// MakeNoZero makes a slice of length n and capacity of at least n Bytes
/// without zeroing the bytes (including the bytes between len and cap).
/// It is the caller's responsibility to ensure uninitialized bytes
/// do not leak to the end user.
pub fn make_no_zero(n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let __n = (*n.lock().unwrap().as_ref().unwrap()).clone();
    if __n < 0 { panic!("internal/bytealg.MakeNoZero: negative length"); }
    let __len = __n as usize;
    Arc::new(Mutex::new(Some(vec![0u8; __len])))
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
