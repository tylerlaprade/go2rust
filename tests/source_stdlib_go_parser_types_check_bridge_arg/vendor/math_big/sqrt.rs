use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone, go_strconv_format_float, go_strconv_format_int};

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
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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


pub(crate) static threeOnce: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct2>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *threeOnce.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *threeOnce.lock().unwrap() = Some(Default::default());
}


impl crate::float::Float {
    /// Sqrt sets z to the rounded square root of x, and returns it.
    ///
    /// If z's precision is 0, it is changed to x's precision before the
    /// operation. Rounding is performed according to z's precision and
    /// rounding mode, but z's accuracy is not computed. Specifically, the
    /// result of z.Acc() is undefined.
    ///
    /// The function panics if z < 0. The value of z is undefined in that
    /// case.
    pub fn sqrt(&mut self, x: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<crate::float::Float>>> {
        if DEBUG_FLOAT {
        { let __recv = x.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.prec.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.sign(); __result }; let __tmp_y = -1; __tmp_x == __tmp_y } {
                // following IEEE754-2008 (section 7.2)
        std::panic::panic_any(Box::new(ErrNaN { msg: Arc::new(Mutex::new(Some("square root of negative operand".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
    }
                // following IEEE754-2008 (section 7.2)
                // handle ±0 and +∞
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::float::form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x != __tmp_y } {
        { let new_val = crate::float::Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = crate::float::form(Arc::new(Mutex::new(Some((*(*(*x.lock().unwrap().as_ref().unwrap()).form.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // IEEE754-2008 requires √±0 = ±0
                // MantExp sets the argument's precision to the receiver's, and
                // when z.prec > x.prec this will lower z.prec. Restore it after
                // the MantExp call.
        let mut prec = Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut b = { let __recv = x.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mant_exp(Arc::new(Mutex::new(Some(self.clone())))); __result };
        { let new_val = prec.lock().unwrap().as_ref().unwrap().clone(); *self.prec.lock().unwrap() = Some(new_val); };
                // Compute √(z·2**b) as
                //   √( z)·2**(½b)     if b is even
                //   √(2z)·2**(⌊½b⌋)   if b > 0 is odd
                //   √(½z)·2**(⌈½b⌉)   if b < 0 is odd
        { let _switch_val = { let __tmp_x = b; let __tmp_y = 2; __tmp_x % __tmp_y };
    if _switch_val == (0) {
        } else if _switch_val == (1) {
            { let __target = self.exp.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if _switch_val == (-1) {
            { let __target = self.exp.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        }
    }
                // nothing to do
                // 0.25 <= z < 2.0
                // Solving 1/x² - z = 0 avoids Quo calls and is faster, especially
                // for high precisions.
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); self.sqrt_inverse(__method_arg0) };
                // re-attach halved exponent
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __tmp_x = b; let __tmp_y = 2; __tmp_x / __tmp_y }))); self.set_mant_exp(__method_arg0, __method_arg1) }
    }

    /// Compute √x (to z.prec precision) by solving
    ///
    ///	1/t² - x = 0
    ///
    /// for t (using Newton's method), and then inverting.
    pub fn sqrt_inverse(&mut self, x: Arc<Mutex<Option<Float>>>) {
                // let
                //   f(t) = 1/t² - x
                // then
                //   g(t) = f(t)/f'(t) = -½t(1 - xt²)
                // and the next guess is given by
                //   t2 = t - g(t) = ½t(3 - xt²)
        let mut u = new_float_1(Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut v = new_float_1(Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut three = three();
        let three_closure_clone = three.clone(); let u_closure_clone = u.clone(); let v_closure_clone = v.clone(); let x_closure_clone = x.clone(); let mut ng = Arc::new(Mutex::new(Some(Box::new(move |t: Arc<Mutex<Option<Float>>>| -> Arc<Mutex<Option<Float>>> {
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*u_closure_clone.lock().unwrap().as_ref().unwrap()).prec.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*v_closure_clone.lock().unwrap().as_ref().unwrap()).prec.lock().unwrap() = Some(new_val); };
        { let __recv = u_closure_clone.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mul(t.clone(), t.clone()); __result };
        { let __recv = u_closure_clone.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mul(x_closure_clone.clone(), u_closure_clone.clone()); __result };
        { let __recv = v_closure_clone.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.sub(three_closure_clone.clone(), u_closure_clone.clone()); __result };
        { let __recv = u_closure_clone.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mul(t.clone(), v_closure_clone.clone()); __result };
        { let __target = (*u_closure_clone.lock().unwrap().as_ref().unwrap()).exp.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        return { let __recv = t.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.set(u_closure_clone.clone()); __result };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> + Send + Sync>)));
                // u = t²
                //   = xt²
                // v = 3 - xt²
                // u = t(3 - xt²)
                //   = ½t(3 - xt²)
        let (mut xf, _) = { let __recv = x.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.float64(); __result };
        let mut sqi = new_float_1(Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __recv = sqi.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.set_float64(Arc::new(Mutex::new(Some({ let __tmp_x = 1.0; let __tmp_y = math::sqrt(Arc::new(Mutex::new(Some(xf)))); __tmp_x / __tmp_y })))); __result };
        let mut prec = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 32 as u32; __tmp_x + __tmp_y })));
    while { let __tmp_x = (*{ let __field = (*sqi.lock().unwrap().as_ref().unwrap()).prec.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __target = (*sqi.lock().unwrap().as_ref().unwrap()).prec.clone(); let __rhs = 2 as u32; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> + Send + Sync> = { let mut __f_guard = ng.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(sqi.clone()) }.clone(); sqi = new_val; };
    }
                // sqi = 1/√x
                // x/√x = √x
        self.mul(x.clone(), sqi.clone());
    }
}

pub fn three() -> Arc<Mutex<Option<crate::float::Float>>> {
    { let __once = (*threeOnce.lock().unwrap().as_ref().unwrap()).once.clone(); __once.r#do(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = new_float(Arc::new(Mutex::new(Some(3.0)))).clone(); (*threeOnce.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>)))) };
    (*threeOnce.lock().unwrap().as_ref().unwrap()).v.clone()
}

/// newFloat returns a new *Float with space for twice the given
/// precision.
pub fn new_float_1(prec2: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<crate::float::Float>>> {
    let mut z = Arc::new(Mutex::new(Some(Float::default())));

        // nat.make ensures the slice length is > 0
    { let new_val = (*(*z.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).make(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*prec2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u32; __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x * __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*z.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap() = __moved_val; };
    return z.clone();
}

#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub once: sync::once::Once,
    pub v: Arc<Mutex<Option<Float>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { once: self.once.clone(), v: self.v.clone() }
    }
}

impl AnonymousStruct2 {
    pub fn r#do(&mut self, _arg0: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
        let embedded_ref = &mut self.once;
        embedded_ref.r#do(_arg0)
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { once: Default::default(), v: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.v.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type threeOnce = AnonymousStruct2;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
