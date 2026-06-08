use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{mpagealloc::{PALLOC_CHUNK_PAGES, pack_palloc_sum, pallocSum}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// pageBits is a bitmap representing one bit per page in a palloc chunk.
#[derive(Debug, Clone)]
pub struct pageBits(pub Arc<Mutex<Option<[u64; 8]>>>);

impl Default for pageBits {
    fn default() -> Self {
        pageBits(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Display for pageBits {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


/// pallocBits is a bitmap that tracks page allocations for at most one
/// palloc chunk.
///
/// The precise representation is an implementation detail, but for the
/// sake of documentation, 0s are free pages and 1s are allocated pages.
#[derive(Debug, Clone)]
pub struct pallocBits(pub Arc<Mutex<Option<pageBits>>>);

impl Default for pallocBits {
    fn default() -> Self {
        pallocBits(Arc::new(Mutex::new(Some(pageBits(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))))))
    }
}

impl Display for pallocBits {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let __inner_guard = self.0.lock().unwrap();
        write!(f, "{}", __inner_guard.as_ref().unwrap())
    }
}


/// pallocData encapsulates pallocBits and a bitmap for
/// whether or not a given page is scavenged in a single
/// structure. It's effectively a pallocBits with
/// additional functionality.
///
/// Update the comment on (*pageAlloc).chunks should this
/// structure change.
#[derive(Debug, Clone)]
pub struct pallocData {
    pub palloc_bits: Arc<Mutex<Option<pallocBits>>>,
    pub scavenged: Arc<Mutex<Option<pageBits>>>,
}

impl pallocData {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.palloc_bits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.scavenged.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            palloc_bits: __go_clone_0_0,
            scavenged: __go_clone_1_0,
        }
    }
}


impl Default for pallocData {
    fn default() -> Self {
        Self { palloc_bits: Arc::new(Mutex::new(Some(pallocBits(Arc::new(Mutex::new(Some(pageBits(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))))))))), scavenged: Arc::new(Mutex::new(Some(pageBits(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))))) }
    }
}

impl std::fmt::Display for pallocData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.palloc_bits.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.scavenged.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for pallocData {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl pageBits {
    /// get returns the value of the i'th bit in the bitmap.
    pub fn get(&self, i: Arc<Mutex<Option<u64>>>) -> u64 {
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = 1 as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap())
    }

    /// block64 returns the 64-bit aligned block of bits containing the i'th bit.
    pub fn block64(&self, i: Arc<Mutex<Option<u64>>>) -> u64 {
        { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() }
    }

    /// set sets bit i of pageBits.
    pub fn set(&mut self, i: Arc<Mutex<Option<u64>>>) {
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = (1 as u64); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] | __rhs; };
    }

    /// setRange sets bits in the range [i, i+n).
    pub fn set_range(&mut self, i: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) {
        let _ = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
                // Fast path for the n == 1 case.
        self.set(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
                // Fast path for the n == 1 case.
                // Set bits [i, j].
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; __tmp_x == __tmp_y } {
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] | __rhs; };
        return;
    }
        let _ = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() };
                // Set leading bits.
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = (!(0 as u64) as u64); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] | __rhs; };
        let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; __tmp_x < __tmp_y } {
        (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = !(0 as u64) as u64;
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Set trailing bits.
        { let __idx = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] | __rhs; };
    }

    /// setAll sets all the bits of b.
    pub fn set_all(&mut self) {
        for i in 0..(8) {
        (*self.0.clone().lock().unwrap().as_mut().unwrap())[(i) as usize] = !(0 as u64) as u64;
    }
    }

    /// setBlock64 sets the 64-bit aligned block of bits containing the i'th bit that
    /// are set in v.
    pub fn set_block64(&mut self, i: Arc<Mutex<Option<u64>>>, v: Arc<Mutex<Option<u64>>>) {
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = (*v.lock().unwrap().as_ref().unwrap()); let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] | __rhs; };
    }

    /// clear clears bit i of pageBits.
    pub fn clear(&mut self, i: Arc<Mutex<Option<u64>>>) {
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = (1 as u64); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] & ! __rhs; };
    }

    /// clearRange clears bits in the range [i, i+n).
    pub fn clear_range(&mut self, i: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) {
        let _ = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
                // Fast path for the n == 1 case.
        self.clear(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
                // Fast path for the n == 1 case.
                // Clear bits [i, j].
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; __tmp_x == __tmp_y } {
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] & ! __rhs; };
        return;
    }
        let _ = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() };
                // Clear leading bits.
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = (!(0 as u64) as u64); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] & ! __rhs; };
        { let __clear_start = ({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }) as usize; let __clear_end = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize; let __clear_holder = self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = 0; } } };
                // Clear trailing bits.
        { let __idx = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }; let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] & ! __rhs; };
    }

    /// clearAll frees all the bits of b.
    pub fn clear_all(&self) {
        { let __clear_start = 0usize; let __clear_end = { let __clear_len_holder = self.0.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = 0; } } };
    }

    /// clearBlock64 clears the 64-bit aligned block of bits containing the i'th bit that
    /// are set in v.
    pub fn clear_block64(&mut self, i: Arc<Mutex<Option<u64>>>, v: Arc<Mutex<Option<u64>>>) {
        { let __idx = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y } as usize; let __rhs = (*v.lock().unwrap().as_ref().unwrap()); let __seq_holder = self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] & ! __rhs; };
    }

    /// popcntRange counts the number of set bits in the
    /// range [i, i+n).
    pub fn popcnt_range(&self, i: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut s: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = 1 as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap());
    }
        let _ = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() };
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some(internal_runtime_sys::ones_count64(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })))) as u64))).lock().unwrap().as_ref().unwrap());
    }
        let _ = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() };
        { let __rhs = (*Arc::new(Mutex::new(Some(internal_runtime_sys::ones_count64(Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y })))) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }; __tmp_x < __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some(internal_runtime_sys::ones_count64(Arc::new(Mutex::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })))) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let __rhs = (*Arc::new(Mutex::new(Some(internal_runtime_sys::ones_count64(Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x % __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })))) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return (*s.lock().unwrap().as_ref().unwrap());
    }
}

impl pallocBits {
    /// summarize returns a packed summary of the bitmap in pallocBits.
    pub fn summarize(&self) -> Arc<Mutex<Option<crate::mpagealloc::pallocSum>>> {
        let mut start: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut most: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut cur: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        const notSetYet: u64 = !(0 as u64);

        { let new_val = notSetYet as u64; *start.lock().unwrap() = Some(new_val); };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 8; __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let __rhs = 64 as u64; let mut guard = cur.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        let mut t = Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)));
        let mut l = Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)));

                // Finish any region spanning the uint64s
        { let __rhs = (*t.lock().unwrap().as_ref().unwrap()); let mut guard = cur.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = notSetYet as u64; __tmp_x == __tmp_y } {
        { let new_val = cur.lock().unwrap().as_ref().unwrap().clone(); *start.lock().unwrap() = Some(new_val); };
    }
        { let new_val = std::cmp::max(({ let __v = (*most.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64), ({ let __v = (*cur.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64)); *most.lock().unwrap() = Some(new_val); };

                // Final region that might span to next uint64
        { let new_val = l.lock().unwrap().as_ref().unwrap().clone(); *cur.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Finish any region spanning the uint64s
                // Final region that might span to next uint64
        if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = notSetYet as u64; __tmp_x == __tmp_y } {
                // Made it all the way through without finding a single 1 bit.
        const n: u64 = ((64 * 8) as u64);

        return pack_palloc_sum(Arc::new(Mutex::new(Some(n as u64))), Arc::new(Mutex::new(Some(n as u64))), Arc::new(Mutex::new(Some(n as u64))));
    }
                // Made it all the way through without finding a single 1 bit.
        { let new_val = std::cmp::max(({ let __v = (*most.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64), ({ let __v = (*cur.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64)); *most.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*most.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = 64; let __tmp_y = 2; __tmp_x - __tmp_y } as u64; __tmp_x >= __tmp_y } {
                // There is no way an internal run of zeros could beat max.
        return pack_palloc_sum(Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = most.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = cur.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // There is no way an internal run of zeros could beat max.
                // Now look inside each uint64 for runs of zeros.
                // All uint64s must be nonzero, or we would have aborted above.
        let mut i = Arc::new(Mutex::new(Some(0)));
    'outer: while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 8; __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));

                // Look inside this uint64. We have a pattern like
                // 000000 1xxxxx1 000000
                // We need to look inside the 1xxxxx1 for any contiguous
                // region of zeros.
                // We already know the trailing zeros are no larger than max. Remove them.
        { let __rhs = { let __tmp_x = internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 63; __tmp_x & __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // Strategy: shrink all runs of zeros by max. If any runs of zero
                // remain, then we've identified a larger maximum zero run.
        let mut p = { let __owned = most.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let mut k = Arc::new(Mutex::new(Some(1 as u64)));
        loop {
                // Shrink all runs of zeros by p places (except the top zeros).
        while { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
                // Shift p ones down into the top of each run of zeros.
        { let __rhs = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63 as u64; __tmp_x & __tmp_y }); __tmp_x >> __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'outer
    }
        break
    }

                // Shift p ones down into the top of each run of zeros.
                // no more zeros (except at the top).
                // Shift k ones down into the top of each run of zeros.
        { let __rhs = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63 as u64; __tmp_x & __tmp_y }); __tmp_x >> __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'outer
    }
        { let __rhs = (*k.lock().unwrap().as_ref().unwrap()); let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

                // We've just doubled the minimum length of 1-runs.
                // This allows us to shift farther in the next iteration.
        { let __rhs = 2 as u64; let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }

                // Shift p ones down into the top of each run of zeros.
                // no more zeros (except at the top).
                // Shift k ones down into the top of each run of zeros.
                // no more zeros (except at the top).
                // We've just doubled the minimum length of 1-runs.
                // This allows us to shift farther in the next iteration.
                // The length of the lowest-order zero run is an increment to our maximum.
        let mut j = Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some(!(*x.lock().unwrap().as_ref().unwrap()))))) as u64)));
        { let __rhs = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63 as u64; __tmp_x & __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let new_val = Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *j.lock().unwrap() = __moved_val; };
        { let __rhs = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63 as u64; __tmp_x & __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = (*j.lock().unwrap().as_ref().unwrap()); let mut guard = most.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'outer
    }
        { let new_val = j.lock().unwrap().as_ref().unwrap().clone(); *p.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Look inside this uint64. We have a pattern like
                // 000000 1xxxxx1 000000
                // We need to look inside the 1xxxxx1 for any contiguous
                // region of zeros.
                // We already know the trailing zeros are no larger than max. Remove them.
                // no more zeros (except at the top).
                // Strategy: shrink all runs of zeros by max. If any runs of zero
                // remain, then we've identified a larger maximum zero run.
                // number of zeros we still need to shrink by.
                // current minimum length of runs of ones in x.
                // Shrink all runs of zeros by p places (except the top zeros).
                // Shift p ones down into the top of each run of zeros.
                // no more zeros (except at the top).
                // Shift k ones down into the top of each run of zeros.
                // no more zeros (except at the top).
                // We've just doubled the minimum length of 1-runs.
                // This allows us to shift farther in the next iteration.
                // The length of the lowest-order zero run is an increment to our maximum.
                // count contiguous trailing ones
                // remove trailing ones
                // count contiguous trailing zeros
                // remove zeros
                // we have a new maximum!
                // no more zeros (except at the top).
                // remove j more zeros from each zero run.
        return pack_palloc_sum(Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = most.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = cur.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// find searches for npages contiguous free pages in pallocBits and returns
    /// the index where that run starts, as well as the index of the first free page
    /// it found in the search. searchIdx represents the first known free page and
    /// where to begin the next search from.
    ///
    /// If find fails to find any free space, it returns an index of ^uint(0) and
    /// the new searchIdx should be ignored.
    ///
    /// Note that if npages == 1, the two returned values will always be identical.
    pub fn find(&self, npages: Arc<Mutex<Option<usize>>>, searchIdx: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
        if { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
        let mut addr = self.find1(Arc::new(Mutex::new(Some({ let __arg_holder = searchIdx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return (addr, addr);
    } else if { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as usize; __tmp_x <= __tmp_y } {
        return self.find_small_n(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = searchIdx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        self.find_large_n(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = searchIdx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// find1 is a helper for find which searches for a single free page
    /// in the pallocBits and returns the index.
    ///
    /// See find for an explanation of the searchIdx parameter.
    pub fn find1(&self, searchIdx: Arc<Mutex<Option<u64>>>) -> u64 {
        let _ = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[(0) as usize].clone() };
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*searchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(8 as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        if { let __tmp_x = !(*x.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        return { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some(!(*x.lock().unwrap().as_ref().unwrap()))))) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        !0 as u64
    }

    /// findSmallN is a helper for find which searches for npages contiguous free pages
    /// in this pallocBits and returns the index where that run of contiguous pages
    /// starts as well as the index of the first free page it finds in its search.
    ///
    /// See find for an explanation of the searchIdx parameter.
    ///
    /// Returns a ^uint(0) index on failure and the new searchIdx should be ignored.
    ///
    /// findSmallN assumes npages <= 64, where any such contiguous run of pages
    /// crosses at most one aligned 64-bit boundary in the bits.
    pub fn find_small_n(&self, npages: Arc<Mutex<Option<usize>>>, searchIdx: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
        let (mut end, mut newSearchIdx) = (Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some(!0 as u64))));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*searchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(8 as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x < __tmp_y } {
        let mut bi = Arc::new(Mutex::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        if { let __tmp_x = !(*bi.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *end.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // First see if we can pack our allocation in the trailing
                // zeros plus the end of the last 64 bits.
        if { let __tmp_x = { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
                // The new searchIdx is going to be at these 64 bits after any
                // 1s we file, so count trailing 1s.
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some(!(*bi.lock().unwrap().as_ref().unwrap()))))) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *newSearchIdx.lock().unwrap() = Some(new_val); };
    }
                // The new searchIdx is going to be at these 64 bits after any
                // 1s we file, so count trailing 1s.
        let mut start = Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = bi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)));
        if { let __tmp_x = { let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return ({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }, { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

                // Next, check the interior of the 64-bit chunk.
        let mut j = find_bit_range64(Arc::new(Mutex::new(Some(!(*bi.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))));
        if { let __tmp_x = j; let __tmp_y = 64 as u64; __tmp_x < __tmp_y } {
        return ({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = j; __tmp_x + __tmp_y }, { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
        { let new_val = Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = bi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *end.lock().unwrap() = __moved_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // First see if we can pack our allocation in the trailing
                // zeros plus the end of the last 64 bits.
                // The new searchIdx is going to be at these 64 bits after any
                // 1s we file, so count trailing 1s.
                // Next, check the interior of the 64-bit chunk.
        return (!0 as u64, { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

    /// findLargeN is a helper for find which searches for npages contiguous free pages
    /// in this pallocBits and returns the index where that run starts, as well as the
    /// index of the first free page it found it its search.
    ///
    /// See alloc for an explanation of the searchIdx parameter.
    ///
    /// Returns a ^uint(0) index on failure and the new searchIdx should be ignored.
    ///
    /// findLargeN assumes npages > 64, where any such run of free pages
    /// crosses at least one aligned 64-bit boundary in the bits.
    pub fn find_large_n(&self, npages: Arc<Mutex<Option<usize>>>, searchIdx: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
        let (mut start, mut size, mut newSearchIdx) = (Arc::new(Mutex::new(Some(!0 as u64))), Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some(!0 as u64))));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*searchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(8 as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *size.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        if { let __tmp_x = { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
                // The new searchIdx is going to be at these 64 bits after any
                // 1s we file, so count trailing 1s.
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some(!(*x.lock().unwrap().as_ref().unwrap()))))) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *newSearchIdx.lock().unwrap() = Some(new_val); };
    }
                // The new searchIdx is going to be at these 64 bits after any
                // 1s we file, so count trailing 1s.
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *size.lock().unwrap() = __moved_val; };
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = 64 as u64; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *start.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        let mut s = Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)));
        if { let __tmp_x = { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *size.lock().unwrap() = __moved_val; };
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = 64 as u64; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *start.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        { let __rhs = 64 as u64; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // The new searchIdx is going to be at these 64 bits after any
                // 1s we file, so count trailing 1s.
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return (!0 as u64, { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
        return ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*newSearchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

    /// allocRange allocates the range [i, i+n).
    pub fn alloc_range(&self, i: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) {
        { let __recv = Arc::new(Mutex::new(Some(pageBits::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_range(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }

    /// allocAll allocates all the bits of b.
    pub fn alloc_all(&self) {
        { let __recv = Arc::new(Mutex::new(Some(pageBits::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_all(); __result };
    }

    /// free1 frees a single page in the pallocBits at i.
    pub fn free1(&self, i: Arc<Mutex<Option<u64>>>) {
        { let __recv = Arc::new(Mutex::new(Some(pageBits::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).clear(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }

    /// free frees the range [i, i+n) of pages in the pallocBits.
    pub fn free(&self, i: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) {
        { let __recv = Arc::new(Mutex::new(Some(pageBits::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).clear_range(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }

    /// freeAll frees all the bits of b.
    pub fn free_all(&self) {
        { let __recv = Arc::new(Mutex::new(Some(pageBits::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).clear_all(); __result };
    }

    /// pages64 returns a 64-bit bitmap representing a block of 64 pages aligned
    /// to 64 pages. The returned block of pages is the one containing the i'th
    /// page in this pallocBits. Each bit represents whether the page is in-use.
    pub fn pages64(&self, i: Arc<Mutex<Option<u64>>>) -> u64 {
        { let __recv = Arc::new(Mutex::new(Some(pageBits::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).block64(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }
    }

    /// allocPages64 allocates a 64-bit block of 64 pages aligned to 64 pages according
    /// to the bits set in alloc. The block set is the one containing the i'th page.
    pub fn alloc_pages64(&self, i: Arc<Mutex<Option<u64>>>, alloc: Arc<Mutex<Option<u64>>>) {
        { let __recv = Arc::new(Mutex::new(Some(pageBits::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_block64(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = alloc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }
}

impl pallocData {
    /// allocRange sets bits [i, i+n) in the bitmap to 1 and
    /// updates the scavenged bits appropriately.
    pub fn alloc_range(&self, i: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) {
                // Clear the scavenged bits when we alloc the range.
        (*self.palloc_bits.lock().unwrap().as_ref().unwrap()).alloc_range(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*self.scavenged.lock().unwrap().as_mut().unwrap()).clear_range(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// allocAll sets every bit in the bitmap to 1 and updates
    /// the scavenged bits appropriately.
    pub fn alloc_all(&self) {
                // Clear the scavenged bits when we alloc the range.
        (*self.palloc_bits.lock().unwrap().as_ref().unwrap()).alloc_all();
        (*self.scavenged.lock().unwrap().as_ref().unwrap()).clear_all();
    }

    pub fn alloc_pages64(&self, i: Arc<Mutex<Option<u64>>>, alloc: Arc<Mutex<Option<u64>>>) {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.alloc_pages64(i, alloc)
    }

    pub fn find(&self, npages: Arc<Mutex<Option<usize>>>, searchIdx: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.find(npages, searchIdx)
    }

    pub fn find1(&self, searchIdx: Arc<Mutex<Option<u64>>>) -> u64 {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.find1(searchIdx)
    }

    pub fn find_large_n(&self, npages: Arc<Mutex<Option<usize>>>, searchIdx: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.find_large_n(npages, searchIdx)
    }

    pub fn find_small_n(&self, npages: Arc<Mutex<Option<usize>>>, searchIdx: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.find_small_n(npages, searchIdx)
    }

    pub fn free(&self, i: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.free(i, n)
    }

    pub fn free1(&self, i: Arc<Mutex<Option<u64>>>) {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.free1(i)
    }

    pub fn free_all(&self) {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.free_all()
    }

    pub fn pages64(&self, i: Arc<Mutex<Option<u64>>>) -> u64 {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pages64(i)
    }

    pub fn summarize(&self) -> Arc<Mutex<Option<crate::mpagealloc::pallocSum>>> {
        // Forward to embedded type's method
        let embedded = self.palloc_bits.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.summarize()
    }
}

/// findBitRange64 returns the bit index of the first set of
/// n consecutive 1 bits. If no consecutive set of 1 bits of
/// size n may be found in c, then it returns an integer >= 64.
/// n must be > 0.
pub fn find_bit_range64(mut c: Arc<Mutex<Option<u64>>>, n: Arc<Mutex<Option<u64>>>) -> u64 {
        // This implementation is based on shrinking the length of
        // runs of contiguous 1 bits. We remove the top n-1 1 bits
        // from each run of 1s, then look for the first remaining 1 bit.
    let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));
    let mut k = Arc::new(Mutex::new(Some(1 as u64)));
    while { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
                // Shift p 0s down into the top of each run of 1s.
        { let __rhs = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63 as u64; __tmp_x & __tmp_y }); __tmp_x >> __tmp_y }; let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
        break
    }

                // Shift p 0s down into the top of each run of 1s.
                // Shift k 0s down into the top of each run of 1s.
        { let __rhs = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63 as u64; __tmp_x & __tmp_y }); __tmp_x >> __tmp_y }; let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return 64;
    }
        { let __rhs = (*k.lock().unwrap().as_ref().unwrap()); let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

                // We've just doubled the minimum length of 0-runs.
                // This allows us to shift farther in the next iteration.
        { let __rhs = 2 as u64; let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }

        // Shift p 0s down into the top of each run of 1s.
        // Shift k 0s down into the top of each run of 1s.
        // We've just doubled the minimum length of 0-runs.
        // This allows us to shift farther in the next iteration.
        // Find first remaining 1.
        // Since we shrunk from the top down, the first 1 is in
        // its correct original position.
    (*Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64))).lock().unwrap().as_ref().unwrap())
}

impl GoValueClone for pallocData {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
