use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_strconv_format_float, go_strconv_format_int};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::float::*;
use crate::floatconv::*;
use crate::floatmarsh::*;
use crate::ftoa::*;
use crate::int::*;
use crate::intconv::*;
use crate::intmarsh::*;
use crate::nat::*;
use crate::natconv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DIV_RECURSIVE_THRESHOLD: i32 = 100;


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub mutex: sync::mutex::Mutex,
    pub table: Arc<Mutex<Option<[divisor; 64]>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), table: { let __guard = self.table.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct1 {
    pub fn lock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.lock()
    }

    pub fn try_lock(&mut self) -> bool {
        let embedded_ref = &mut self.mutex;
        embedded_ref.try_lock()
    }

    pub fn unlock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.unlock()
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { mutex: Default::default(), table: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.table))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type cacheBase10 = AnonymousStruct1;


impl crate::nat::nat {
    /// rem returns r such that r = u%v.
    /// It uses z as the storage for r.
    pub fn rem(&self, u: Arc<Mutex<Option<nat>>>, v: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<crate::nat::nat>>> {
    let mut r: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));

        let mut __self = self.clone();
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        *__self.0.lock().unwrap() = None;
    }
        let mut qp = get_nat(Arc::new(Mutex::new(Some(0))));
        let (mut q, __tmp_1) = { let __recv = qp.clone(); let __recv_ptr: *const crate::nat::nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::nat::nat }; let __result = unsafe { &*__recv_ptr }.div(Arc::new(Mutex::new(Some(__self.clone()))), u.clone(), v.clone()); __result }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1;;
        { let new_val = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; *qp.lock().unwrap() = Some(new_val); };
        put_nat(qp.clone());
        return r.clone();
    }

    /// div returns q, r such that q = ⌊u/v⌋ and r = u%v = u - q·v.
    /// It uses z and z2 as the storage for q and r.
    pub fn div(&self, z2: Arc<Mutex<Option<nat>>>, u: Arc<Mutex<Option<nat>>>, v: Arc<Mutex<Option<nat>>>) -> (Arc<Mutex<Option<crate::nat::nat>>>, Arc<Mutex<Option<crate::nat::nat>>>) {
    let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut r: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));

        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }
        if { let __tmp_x = (*u.lock().unwrap().as_ref().unwrap()).cmp(v.clone()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..(0) as usize].to_vec() })))); *q.lock().unwrap() = Some(new_val); };
        { let new_val = (*z2.lock().unwrap().as_ref().unwrap()).set(u.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
        return (q.clone(), r.clone());
    }
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
                // Short division: long optimized for a single-word divisor.
                // In that case, the 2-by-1 guess is all we need at each step.
        let mut r2: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        { let (__tmp_0, __tmp_1) = self.div_w(u.clone(), Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r2.lock().unwrap() = __moved_tmp_1; };
        { let new_val = (*z2.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = r2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
        return (q.clone(), r.clone());
    }
                // Short division: long optimized for a single-word divisor.
                // In that case, the 2-by-1 guess is all we need at each step.
        { let (__tmp_0, __tmp_1) = self.div_large(z2.clone(), u.clone(), v.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };
        (q.clone(), r.clone())
    }

    /// divW returns q, r such that q = ⌊x/y⌋ and r = x%y = x - q·y.
    /// It uses z as the storage for q.
    /// Note that y is a single digit (Word), not a big number.
    pub fn div_w(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<Word>>>) -> (Arc<Mutex<Option<crate::nat::nat>>>, Arc<Mutex<Option<crate::arith::Word>>>) {
    let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut r: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));

        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = (*y.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
            std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
        } else if { let __tmp_x = (*y.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x == __tmp_y } {
            { let new_val = __self.set(x.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
            return (q.clone(), r.clone());
        } else if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            { let new_val = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..(0) as usize].to_vec() })))); *q.lock().unwrap() = Some(new_val); };
            return (q.clone(), r.clone());
        }
                // result is x
                // result is 0
                // m > 0
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let new_val = div_w_v_w(__self.0.clone(), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64))))))), { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
        { let new_val = __self.norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
        (q.clone(), r.clone())
    }

    /// modW returns x % d.
    pub fn mod_w(&self, d: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    let mut r: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));

                // TODO(agl): we don't actually need to store the q value.
        let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = (*q.lock().unwrap().as_ref().unwrap()).make(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
        return div_w_v_w({ let __named_slice = (*q.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64))))))), self.0.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// div returns q, r such that q = ⌊uIn/vIn⌋ and r = uIn%vIn = uIn - q·vIn.
    /// It uses z and u as the storage for q and r.
    /// The caller must ensure that len(vIn) ≥ 2 (use divW otherwise)
    /// and that len(uIn) ≥ len(vIn) (the answer is 0, uIn otherwise).
    pub fn div_large(&self, mut u: Arc<Mutex<Option<nat>>>, uIn: Arc<Mutex<Option<nat>>>, vIn: Arc<Mutex<Option<nat>>>) -> (Arc<Mutex<Option<crate::nat::nat>>>, Arc<Mutex<Option<crate::nat::nat>>>) {
    let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut r: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));

        let mut __self = self.clone();
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*vIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*uIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y })));
                // Scale the inputs so vIn's top bit is 1 (see “Scaling Inputs” above).
                // vIn is treated as a read-only input (it may be in use by another
                // goroutine), so we must make a copy.
                // uIn is copied to u.
        let mut shift = nlz(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*vIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))));
        let mut vp = get_nat(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut v = Arc::new(Mutex::new(Some({ let __v = (*vp.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        shl_v_u({ let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*vIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(shift))));
        { let new_val = (*u.lock().unwrap().as_ref().unwrap()).make(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*uIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *u.lock().unwrap() = __moved_val; };
        (*{ let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __slice_holder = { let __named_slice = (*uIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*shl_v_u({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __slice_holder = { let __named_slice = (*uIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[..__high].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = (*uIn.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(shift)))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
                // The caller should not pass aliased z and u, since those are
                // the two different outputs, but correct just in case.
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        *__self.0.lock().unwrap() = None;
    }
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
                // Use basic or recursive long division depending on size.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x < __tmp_y } {
        (*q.lock().unwrap().as_ref().unwrap()).div_basic(u.clone(), v.clone());
    } else {
        (*q.lock().unwrap().as_ref().unwrap()).div_recursive(u.clone(), v.clone());
    }
        put_nat(vp.clone());
        { let new_val = (*q.lock().unwrap().as_ref().unwrap()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
                // Undo scaling of remainder.
        shr_v_u({ let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(shift))));
        { let new_val = (*u.lock().unwrap().as_ref().unwrap()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
        return (q.clone(), r.clone());
    }

    /// divBasic implements long division as described above.
    /// It overwrites q with ⌊u/v⌋ and overwrites u with the remainder r.
    /// q must be large enough to hold ⌊u/v⌋.
    pub fn div_basic(&self, u: Arc<Mutex<Option<nat>>>, v: Arc<Mutex<Option<nat>>>) {
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y })));
        let mut qhatvp = get_nat(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))));
        let mut qhatv = Arc::new(Mutex::new(Some({ let __v = (*qhatvp.lock().unwrap().as_ref().unwrap()).clone(); __v })));
                // Set up for divWW below, precomputing reciprocal argument.
        let mut vn1 = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        let mut rec = reciprocal_word(Arc::new(Mutex::new(Some({ let __arg_holder = vn1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Invent a leading 0 for u, for the first iteration.
                // Invariant: ujn == u[j+n] in each iteration.
        let mut ujn = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))))));
                // Compute each digit of quotient.
        let mut j = { let __owned = m.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
                // Compute the 2-by-1 guess q̂.
        let mut qhat = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(__M as u64 as u64)))))));

                // ujn ≤ vn1, or else q̂ would be more than one digit.
                // For ujn == vn1, we set q̂ to the max digit M above.
                // Otherwise, we compute the 2-by-1 guess.
        if { let __tmp_x = (*ujn.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*vn1.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        let mut rhat: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        { let (__tmp_0, __tmp_1) = div_w_w(Arc::new(Mutex::new(Some({ let __arg_holder = ujn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some({ let __arg_holder = vn1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *qhat.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *rhat.lock().unwrap() = __moved_tmp_1; };
                // Refine q̂ to a 3-by-2 guess. See “Refining Guesses” above.
        let mut vn2 = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        let (mut x1, mut x2) = mul_w_w(Arc::new(Mutex::new(Some({ let __arg_holder = qhat.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = vn2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut ujn2 = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        while greater_than(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = x2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rhat.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ujn2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = qhat.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - 1 as u64); }
        let mut prevRhat = { let __owned = rhat.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        { let __rhs = (*({ let __v = (*vn1.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = rhat.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };

                // If r̂  overflows, then
                // r̂ u[j+n-2]v[n-1] is now definitely > x1 x2.
        if { let __tmp_x = (*rhat.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*prevRhat.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
        break
    }

                // TODO(rsc): No need for a full mulWW.
                // x2 += vn2; if x2 overflows, x1++
        { let (__tmp_0, __tmp_1) = mul_w_w(Arc::new(Mutex::new(Some({ let __arg_holder = qhat.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = vn2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x1.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x2.lock().unwrap() = __moved_tmp_1; };
    }
    }

                // Refine q̂ to a 3-by-2 guess. See “Refining Guesses” above.
                // x1x2 > r̂ u[j+n-2]
                // If r̂  overflows, then
                // r̂ u[j+n-2]v[n-1] is now definitely > x1 x2.
                // TODO(rsc): No need for a full mulWW.
                // x2 += vn2; if x2 overflows, x1++
                // Compute q̂·v.
        (*{ let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*mul_add_v_w_w({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[(0) as usize..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __arg_holder = qhat.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))))))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
        let mut qhl = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*qhl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __seq_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let mut guard = qhl.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

                // Subtract q̂·v from the current section of u.
                // If it underflows, q̂·v > u, which we fix up
                // by decrementing q̂ and adding v back.
        let mut c = sub_v_v({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*qhl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        let mut c = add_v_v({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
                // If n == qhl, the carry from subVV and the carry from addVV
                // cancel out and don't affect u[j+n].
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*qhl.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __idx = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as usize; let __rhs = (*({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let __seq_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + __rhs; };
    }
        { let mut guard = qhat.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - 1 as u64); }
    }

                // If n == qhl, the carry from subVV and the carry from addVV
                // cancel out and don't affect u[j+n].
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))); *ujn.lock().unwrap() = Some(new_val); };

                // Save quotient digit.
                // Caller may know the top digit is zero and not leave room for it.
        if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = ({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x == __tmp_y } && { let __tmp_x = (*qhat.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }; continue
    }
        (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __v = (*qhat.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // Compute the 2-by-1 guess q̂.
                // ujn ≤ vn1, or else q̂ would be more than one digit.
                // For ujn == vn1, we set q̂ to the max digit M above.
                // Otherwise, we compute the 2-by-1 guess.
                // Refine q̂ to a 3-by-2 guess. See “Refining Guesses” above.
                // x1x2 > r̂ u[j+n-2]
                // If r̂  overflows, then
                // r̂ u[j+n-2]v[n-1] is now definitely > x1 x2.
                // TODO(rsc): No need for a full mulWW.
                // x2 += vn2; if x2 overflows, x1++
                // Compute q̂·v.
                // Subtract q̂·v from the current section of u.
                // If it underflows, q̂·v > u, which we fix up
                // by decrementing q̂ and adding v back.
                // If n == qhl, the carry from subVV and the carry from addVV
                // cancel out and don't affect u[j+n].
                // Save quotient digit.
                // Caller may know the top digit is zero and not leave room for it.
        put_nat(qhatvp.clone());
    }

    /// divRecursive implements recursive division as described above.
    /// It overwrites z with ⌊u/v⌋ and overwrites u with the remainder r.
    /// z must be large enough to hold ⌊u/v⌋.
    /// This function is just for allocating and freeing temporaries
    /// around divRecursiveStep, the real implementation.
    pub fn div_recursive(&self, u: Arc<Mutex<Option<nat>>>, v: Arc<Mutex<Option<nat>>>) {
                // Recursion depth is (much) less than 2 log₂(len(v)).
                // Allocate a slice of temporaries to be reused across recursion,
                // plus one extra temporary not live across the recursion.
        let mut recDepth = Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = math_bits::len(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64)))); __tmp_x * __tmp_y })));
        let mut tmp = get_nat(Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = ({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x * __tmp_y }))));
        let mut temps: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::nat::nat>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __v = (*recDepth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
        { let __clear_holder = self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        self.div_recursive_step(u.clone(), v.clone(), Arc::new(Mutex::new(Some(0))), tmp.clone(), temps.clone());
                // Free temporaries.
        { let __range_holder = temps.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        if (*n.lock().unwrap()).is_some() {
        put_nat((*n).clone());
    }
    } }
        put_nat(tmp.clone());
    }

    /// divRecursiveStep is the actual implementation of recursive division.
    /// It adds ⌊u/v⌋ to z and overwrites u with the remainder r.
    /// z must be large enough to hold ⌊u/v⌋.
    /// It uses temps[depth] (allocating if needed) as a temporary live across
    /// the recursive call. It also uses tmp, but not live across the recursion.
    pub fn div_recursive_step(&self, mut u: Arc<Mutex<Option<nat>>>, mut v: Arc<Mutex<Option<nat>>>, depth: Arc<Mutex<Option<i32>>>, tmp: Arc<Mutex<Option<nat>>>, temps: Arc<Mutex<Option<Vec<Arc<Mutex<Option<nat>>>>>>>) {
                // u is a subsection of the original and may have leading zeros.
                // TODO(rsc): The v = v.norm() is useless and should be removed.
                // We know (and require) that v's top digit is ≥ B/2.
        { let new_val = (*u.lock().unwrap().as_ref().unwrap()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *u.lock().unwrap() = __moved_val; };
        { let new_val = (*v.lock().unwrap().as_ref().unwrap()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __clear_holder = self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        return;
    }
                // Fall back to basic division if the problem is now small enough.
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x < __tmp_y } {
        self.div_basic(u.clone(), v.clone());
        return;
    }
                // Nothing to do if u is shorter than v (implies u < v).
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return;
    }
                // We consider B digits in a row as a single wide digit.
                // (See “Recursive Division” above.)
                //
                // TODO(rsc): rename B to Wide, to avoid confusion with _B,
                // which is something entirely different.
                // TODO(rsc): Look into whether using ⌈n/2⌉ is better than ⌊n/2⌋.
        let mut B = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x / __tmp_y })));
                // Allocate a nat for qhat below.
        if (*{ let __seq = { let __seq_holder = temps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap()).is_none() {
        (*temps.lock().unwrap().as_mut().unwrap())[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = get_nat(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        { let new_val = (*{ let __recv = { let __seq = { let __seq_holder = temps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *{ let __seq = { let __seq_holder = temps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap() = Some(new_val); };
    }
                // TODO(rsc): Can be just B+1.
                // Compute each wide digit of the quotient.
                //
                // TODO(rsc): Change the loop to be
                //	for j := (m+B-1)/B*B; j > 0; j -= B {
                // which will make the final step a regular step, letting us
                // delete what amounts to an extra copy of the loop body below.
        let mut j = { let __owned = m.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // Divide u[j-B:j+n] (3 wide digits) by v (2 wide digits).
                // First make the 2-by-1-wide-digit guess using a recursive call.
                // Then extend the guess to the full 3-by-2 (see “Refining Guesses”).
                //
                // For the 2-by-1-wide-digit guess, instead of doing 2B-by-B-digit,
                // we use a (2B+1)-by-(B+1) digit, which handles the possibility that
                // the result has an extra leading 1 digit as well as guaranteeing
                // that the computed q̂ will be off by at most 1 instead of 2.
                // s is the number of digits to drop from the 3B- and 2B-digit chunks.
                // We drop B-1 to be left with 2B+1 and B+1.
        let mut s = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))));

                // uu is the up-to-3B-digit section of u we are working on.
        let mut uu = Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __low = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize; __seq[__low..].to_vec() })))))));

                // Compute the 2-by-1 guess q̂, leaving r̂ in uu[s:B+n].
        let mut qhat = Arc::new(Mutex::new(Some({ let __v = (*{ let __seq = { let __seq_holder = temps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let __clear_holder = { let __named_slice = (*qhat.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        (*qhat.lock().unwrap().as_ref().unwrap()).div_recursive_step(Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*uu.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __tmp_x = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() }))))))), Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))))))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), tmp.clone(), temps.clone());
        { let new_val = (*qhat.lock().unwrap().as_ref().unwrap()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *qhat.lock().unwrap() = __moved_val; };

                // Extend to a 3-by-2 quotient and remainder.
                // Because divRecursiveStep overwrote the top part of uu with
                // the remainder r̂, the full uu already contains the equivalent
                // of r̂·B + uₙ₋₂ from the “Refining Guesses” discussion.
                // Subtracting q̂·vₙ₋₂ from it will compute the full-length remainder.
                // If that subtraction underflows, q̂·v > u, which we fix up
                // by decrementing q̂ and adding v back, same as in long division.
                // TODO(rsc): Instead of subtract and fix-up, this code is computing
                // q̂·vₙ₋₂ and decrementing q̂ until that product is ≤ u.
                // But we can do the subtraction directly, as in the comment above
                // and in long division, because we know that q̂ is wrong by at most one.
        let mut qhatv = { let __recv = tmp.clone(); let __recv_ptr: *const crate::nat::nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::nat::nat }; let __result = unsafe { &*__recv_ptr }.make(Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })))); __result };
        { let __clear_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        { let new_val = (*qhatv.lock().unwrap().as_ref().unwrap()).mul(qhat.clone(), Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *qhatv.lock().unwrap() = __moved_val; };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        let mut e = (*qhatv.lock().unwrap().as_ref().unwrap()).cmp((*uu.lock().unwrap().as_ref().unwrap()).norm());
        if { let __tmp_x = e; let __tmp_y = 0; __tmp_x <= __tmp_y } {
        break
    }
        sub_v_w({ let __named_slice = (*qhat.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*qhat.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))))));
        let mut c = sub_v_v({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); __named_slice.0.clone() });
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x > __tmp_y } {
        sub_v_w({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        add_at(Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*uu.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))))))), Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))))))), Arc::new(Mutex::new(Some(0))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = (*qhatv.lock().unwrap().as_ref().unwrap()).cmp((*uu.lock().unwrap().as_ref().unwrap()).norm()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("impossible".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut c = sub_v_v({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*uu.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[..__high].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*uu.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[..__high].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x > __tmp_y } {
        sub_v_w({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*uu.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __low = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[__low..].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*uu.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __low = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[__low..].to_vec() })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        add_at(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = qhat.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
        { let __rhs = (*B.lock().unwrap().as_ref().unwrap()); let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Divide u[j-B:j+n] (3 wide digits) by v (2 wide digits).
                // First make the 2-by-1-wide-digit guess using a recursive call.
                // Then extend the guess to the full 3-by-2 (see “Refining Guesses”).
                //
                // For the 2-by-1-wide-digit guess, instead of doing 2B-by-B-digit,
                // we use a (2B+1)-by-(B+1) digit, which handles the possibility that
                // the result has an extra leading 1 digit as well as guaranteeing
                // that the computed q̂ will be off by at most 1 instead of 2.
                // s is the number of digits to drop from the 3B- and 2B-digit chunks.
                // We drop B-1 to be left with 2B+1 and B+1.
                // uu is the up-to-3B-digit section of u we are working on.
                // Compute the 2-by-1 guess q̂, leaving r̂ in uu[s:B+n].
                // Extend to a 3-by-2 quotient and remainder.
                // Because divRecursiveStep overwrote the top part of uu with
                // the remainder r̂, the full uu already contains the equivalent
                // of r̂·B + uₙ₋₂ from the “Refining Guesses” discussion.
                // Subtracting q̂·vₙ₋₂ from it will compute the full-length remainder.
                // If that subtraction underflows, q̂·v > u, which we fix up
                // by decrementing q̂ and adding v back, same as in long division.
                // TODO(rsc): Instead of subtract and fix-up, this code is computing
                // q̂·vₙ₋₂ and decrementing q̂ until that product is ≤ u.
                // But we can do the subtraction directly, as in the comment above
                // and in long division, because we know that q̂ is wrong by at most one.
                // TODO(rsc): Rewrite loop as described above and delete all this code.
                // Now u < (v<<B), compute lower bits in the same way.
                // Choose shift = B-1 again.
        let mut s = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut qhat = Arc::new(Mutex::new(Some({ let __v = (*{ let __seq = { let __seq_holder = temps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let __clear_holder = { let __named_slice = (*qhat.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        (*qhat.lock().unwrap().as_ref().unwrap()).div_recursive_step(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))).norm(), Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))))))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), tmp.clone(), temps.clone());
        { let new_val = (*qhat.lock().unwrap().as_ref().unwrap()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *qhat.lock().unwrap() = __moved_val; };
        let mut qhatv = { let __recv = tmp.clone(); let __recv_ptr: *const crate::nat::nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::nat::nat }; let __result = unsafe { &*__recv_ptr }.make(Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })))); __result };
        { let __clear_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        { let new_val = (*qhatv.lock().unwrap().as_ref().unwrap()).mul(qhat.clone(), Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *qhatv.lock().unwrap() = __moved_val; };
                // Set the correct remainder as before.
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        {
        let mut e = (*qhatv.lock().unwrap().as_ref().unwrap()).cmp((*u.lock().unwrap().as_ref().unwrap()).norm());;
        if { let __tmp_x = e; let __tmp_y = 0; __tmp_x > __tmp_y } {
            sub_v_w({ let __named_slice = (*qhat.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*qhat.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))))));;
            let mut c = sub_v_v({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); __named_slice.0.clone() });;
            if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x > __tmp_y } {
        sub_v_w({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    };
            add_at(Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))))))), Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))))))), Arc::new(Mutex::new(Some(0))));;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = (*qhatv.lock().unwrap().as_ref().unwrap()).cmp((*u.lock().unwrap().as_ref().unwrap()).norm()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("impossible".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut c = sub_v_v({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[..__high].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[..__high].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x > __tmp_y } {
        { let new_val = sub_v_w({ let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __low = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[__low..].to_vec() })))); __named_slice.0.clone() }, { let __named_slice = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*u.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __low = ({ let __slice_holder = { let __named_slice = (*qhatv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize; __seq[__low..].to_vec() })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *c.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("impossible".to_string()) as Box<dyn Any + Send + Sync>);
    }
                // Done!
        add_at(Arc::new(Mutex::new(Some(self.clone()))), (*qhat.lock().unwrap().as_ref().unwrap()).norm(), Arc::new(Mutex::new(Some(0))));
    }
}

/// divWVW overwrites z with ⌊x/y⌋, returning the remainder r.
/// The caller must ensure that len(z) = len(x).
pub fn div_w_v_w(z: Arc<Mutex<Option<Vec<Word>>>>, xn: Arc<Mutex<Option<Word>>>, x: Arc<Mutex<Option<Vec<Word>>>>, y: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    let mut r: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));

    { let new_val = xn.lock().unwrap().as_ref().unwrap().clone(); *r.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = ((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        let (mut qq, mut rr) = math_bits::div(Arc::new(Mutex::new(Some((*{ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))));
        (*z.lock().unwrap().as_mut().unwrap())[(0) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(qq as u64))));
        return Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(rr as u64)))))));
    }
    let mut rec = reciprocal_word(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*z.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let (__tmp_0, __tmp_1) = div_w_w(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); (*z.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0.lock().unwrap().take().unwrap_or_default(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __owned = r.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// greaterThan reports whether the two digit numbers x1 x2 > y1 y2.
/// TODO(rsc): In contradiction to most of this file, x1 is the high
/// digit and x2 is the low digit. This should be fixed.
pub fn greater_than(x1: Arc<Mutex<Option<Word>>>, x2: Arc<Mutex<Option<Word>>>, y1: Arc<Mutex<Option<Word>>>, y2: Arc<Mutex<Option<Word>>>) -> bool {
    return { let __tmp_x = (*x1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y1.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y } || { let __tmp_x = (*x1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y1.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && { let __tmp_x = (*x2.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y };
}