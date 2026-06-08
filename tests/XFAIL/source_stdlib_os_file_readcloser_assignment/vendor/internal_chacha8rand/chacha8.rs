use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const CTR_INC: i32 = 4;
pub(crate) const CTR_MAX: i32 = 16;
pub(crate) const CHUNK: i32 = 32;
pub(crate) const RESEED: i32 = 4;


/// A State holds the state for a single random generator.
/// It must be used from one goroutine at a time.
/// If used by multiple goroutines at a time, the goroutines
/// may see the same random values, but the code will not
/// crash or cause out-of-bounds memory accesses.
#[derive(Debug, Clone)]
pub struct State {
    pub buf: Arc<Mutex<Option<[u64; 32]>>>,
    pub seed: Arc<Mutex<Option<[u64; 4]>>>,
    pub i: Arc<Mutex<Option<u32>>>,
    pub n: Arc<Mutex<Option<u32>>>,
    pub c: Arc<Mutex<Option<u32>>>,
}

impl State {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.seed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            buf: __go_clone_0_0,
            seed: __go_clone_1_0,
            i: __go_clone_2_0,
            n: __go_clone_3_0,
            c: __go_clone_4_0,
        }
    }
}


impl Default for State {
    fn default() -> Self {
        Self { buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), seed: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), i: Arc::new(Mutex::new(Some(0))), n: Arc::new(Mutex::new(Some(0))), c: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", format_slice(&self.buf), format_slice(&self.seed), (*self.i.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()), (*self.c.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for State {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl State {
    /// Next returns the next random value, along with a boolean
    /// indicating whether one was available.
    /// If one is not available, the caller should call Refill
    /// and then repeat the call to Next.
    ///
    /// Next is //go:nosplit to allow its use in the runtime
    /// with per-m data without holding the per-m lock.
    ///
    ///go:nosplit
    pub fn next(&mut self) -> (u64, bool) {
        let mut i = Arc::new(Mutex::new(Some({ let __selector_holder = self.i.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.n.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, false);
    }
        { let new_val = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }; *self.i.lock().unwrap() = Some(new_val); };
        return ({ let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31 as u32; __tmp_x & __tmp_y }) as usize].clone() }, true);
    }

    /// Init seeds the State with the given seed value.
    pub fn init(&mut self, seed: Arc<Mutex<Option<[u8; 32]>>>) {
        self.init64(Arc::new(Mutex::new(Some([internal_byteorder::l_e_uint64(Arc::new(Mutex::new(Some({ let __seq_holder = seed.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __tmp_x = 0; let __tmp_y = 8; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))), internal_byteorder::l_e_uint64(Arc::new(Mutex::new(Some({ let __seq_holder = seed.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __tmp_x = 1; let __tmp_y = 8; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))), internal_byteorder::l_e_uint64(Arc::new(Mutex::new(Some({ let __seq_holder = seed.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __tmp_x = 2; let __tmp_y = 8; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))), internal_byteorder::l_e_uint64(Arc::new(Mutex::new(Some({ let __seq_holder = seed.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __tmp_x = 3; let __tmp_y = 8; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))]))));
    }

    /// Init64 seeds the state with the given seed value.
    pub fn init64(&mut self, seed: Arc<Mutex<Option<[u64; 4]>>>) {
        { let new_val = seed.lock().unwrap().as_ref().unwrap().clone(); *self.seed.lock().unwrap() = Some(new_val); };
        block(self.seed.clone(), self.buf.clone(), Arc::new(Mutex::new(Some(0 as u32))));
        { let new_val = 0 as u32; *self.c.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u32; *self.i.lock().unwrap() = Some(new_val); };
        { let new_val = CHUNK as u32; *self.n.lock().unwrap() = Some(new_val); };
    }

    /// Refill refills the state with more random values.
    /// After a call to Refill, an immediate call to Next will succeed
    /// (unless multiple goroutines are incorrectly sharing a state).
    pub fn refill(&mut self) {
        { let __target = self.c.clone(); let __rhs = CTR_INC as u32; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = (*self.c.lock().unwrap().as_ref().unwrap()); let __tmp_y = CTR_MAX as u32; __tmp_x == __tmp_y } {
                // Reseed with generated uint64s for forward secrecy.
                // Normally this is done immediately after computing a block,
                // but we do it immediately before computing the next block,
                // to allow a much smaller serialized state (just the seed plus offset).
                // This gives a delayed benefit for the forward secrecy
                // (you can reconstruct the recent past given a memory dump),
                // which we deem acceptable in exchange for the reduced size.
        (*self.seed.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 28; let __tmp_y = 0; __tmp_x + __tmp_y }) as usize].clone() };
        (*self.seed.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 28; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() };
        (*self.seed.lock().unwrap().as_mut().unwrap())[(2) as usize] = { let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 28; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize].clone() };
        (*self.seed.lock().unwrap().as_mut().unwrap())[(3) as usize] = { let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 28; let __tmp_y = 3; __tmp_x + __tmp_y }) as usize].clone() };
        { let new_val = 0 as u32; *self.c.lock().unwrap() = Some(new_val); };
    }
                // Reseed with generated uint64s for forward secrecy.
                // Normally this is done immediately after computing a block,
                // but we do it immediately before computing the next block,
                // to allow a much smaller serialized state (just the seed plus offset).
                // This gives a delayed benefit for the forward secrecy
                // (you can reconstruct the recent past given a memory dump),
                // which we deem acceptable in exchange for the reduced size.
        block(self.seed.clone(), self.buf.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as u32; *self.i.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some((*self.buf.lock().unwrap().as_ref().unwrap()).len() as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.n.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*self.c.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((CTR_MAX as u32) - (CTR_INC as u32)) as u32; __tmp_x == __tmp_y } {
        { let new_val = (((32 as u32) as u32) - (RESEED as u32)) as u32; *self.n.lock().unwrap() = Some(new_val); };
    }
    }

    /// Reseed reseeds the state with new random values.
    /// After a call to Reseed, any previously returned random values
    /// have been erased from the memory of the state and cannot be
    /// recovered.
    pub fn reseed(&mut self) {
        let mut seed: Arc<Mutex<Option<[u64; 4]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        for i in 0..(({ let __range_holder = seed.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        loop {
        let (mut x, mut ok) = self.next();
        if ok {
        (*seed.lock().unwrap().as_mut().unwrap())[(i) as usize] = x;
        break
    }
        self.refill();
    }
    }
        self.init64(Arc::new(Mutex::new(Some({ let __arg_holder = seed.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

/// block is the chacha8rand block function.
pub fn block(seed: Arc<Mutex<Option<[u64; 4]>>>, blocks: Arc<Mutex<Option<[u64; 32]>>>, counter: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


impl GoValueClone for State {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
