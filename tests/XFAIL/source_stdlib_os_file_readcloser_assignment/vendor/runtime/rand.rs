use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{lock_spinbit::{lock, unlock}, panic::{fatal}, runtime2::{g, m, mutex}, stubs::{getg}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) static startupRand: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static globalRand: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct24>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static readRandomFailed: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *startupRand.lock().unwrap() = Some(vec![]);
    *globalRand.lock().unwrap() = Some(Default::default());
    *readRandomFailed.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *startupRand.lock().unwrap() = Some(vec![]);
    *globalRand.lock().unwrap() = Some(Default::default());
    *readRandomFailed.lock().unwrap() = Some(false);
}


/// bootstrapRand returns a random uint64 from the global random generator.
pub fn bootstrap_rand() -> u64 {
    lock(GoPtr::local((*globalRand.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if !(*{ let __field = (*globalRand.lock().unwrap().as_ref().unwrap()).init.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        fatal(Arc::new(Mutex::new(Some("randinit missed".to_string()))));
    }
    loop {
        {
        let (mut x, mut ok) = (*(*globalRand.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_mut().unwrap()).next();;
        if ok {
            unlock(GoPtr::local((*globalRand.lock().unwrap().as_ref().unwrap()).lock.clone()));;
            return x;;
        }
    }
        (*(*globalRand.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_mut().unwrap()).refill();
    }
}

/// bootstrapRandReseed reseeds the bootstrap random number generator,
/// clearing from memory any trace of previously returned random numbers.
pub fn bootstrap_rand_reseed() {
    lock(GoPtr::local((*globalRand.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if !(*{ let __field = (*globalRand.lock().unwrap().as_ref().unwrap()).init.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        fatal(Arc::new(Mutex::new(Some("randinit missed".to_string()))));
    }
    (*(*globalRand.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_mut().unwrap()).reseed();
    unlock(GoPtr::local((*globalRand.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// rand returns a random uint64 from the per-m chacha8 state.
/// This is called from compiler-generated code.
///
/// Do not change signature: used via linkname from other packages.
///
///go:nosplit
///go:linkname rand
pub fn rand() -> u64 {
        // Note: We avoid acquirem here so that in the fast path
        // there is just a getg, an inlined c.Next, and a return.
        // The performance difference on a 16-core AMD is
        // 3.7ns/call this way versus 4.3ns/call with acquirem (+16%).
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
    let mut c = (*mp.lock().unwrap().as_ref().unwrap()).chacha8.clone();
    loop {
                // Note: c.Next is marked nosplit,
                // so we don't need to use mp.locks
                // on the fast path, which is that the
                // first attempt succeeds.
        let (mut x, mut ok) = { let __recv = c.clone(); let __recv_ptr: *mut internal_chacha8rand::chacha8::State = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_chacha8rand::chacha8::State }; let __result = unsafe { &mut *__recv_ptr }.next(); __result };
        if ok {
        return x;
    }
        { let __target = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __recv = c.clone(); let __recv_ptr: *mut internal_chacha8rand::chacha8::State = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_chacha8rand::chacha8::State }; let __result = unsafe { &mut *__recv_ptr }.refill(); __result };
        { let __target = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

/// mrandinit initializes the random state of an m.
pub fn mrandinit(mp: Arc<Mutex<Option<m>>>) {
    let mut seed: Arc<Mutex<Option<[u64; 4]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    for i in 0..(({ let __range_holder = seed.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*seed.lock().unwrap().as_mut().unwrap())[(i) as usize] = bootstrap_rand();
    }
    bootstrap_rand_reseed();
    (*(*mp.lock().unwrap().as_ref().unwrap()).chacha8.lock().unwrap().as_mut().unwrap()).init64(Arc::new(Mutex::new(Some({ let __arg_holder = seed.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = rand(); *(*mp.lock().unwrap().as_ref().unwrap()).cheaprand.lock().unwrap() = Some(new_val); };
}

/// randn is like rand() % n but faster.
/// Do not change signature: used via linkname from other packages.
///
///go:nosplit
///go:linkname randn
pub fn randn(n: Arc<Mutex<Option<u32>>>) -> u32 {
        // See https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
    (*Arc::new(Mutex::new(Some(({
        let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some(rand() as u32 as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y });
        let __tmp_y = 32;
        __tmp_x >> __tmp_y
    }) as u32))).lock().unwrap().as_ref().unwrap())
}

/// cheaprand is a non-cryptographic-quality 32-bit random generator
/// suitable for calling at very high frequency (such as during scheduling decisions)
/// and at sensitive moments in the runtime (such as during stack unwinding).
/// it is "cheap" in the sense of both expense and quality.
///
/// cheaprand must not be exported to other packages:
/// the rule is that other packages using runtime-provided
/// randomness must always use rand.
///
/// cheaprand should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/gopkg
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname cheaprand
///go:nosplit
pub fn cheaprand() -> u32 {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();

        // Implement wyrand: https://github.com/wangyi-fudan/wyhash
        // Only the platform that math.Mul64 can be lowered
        // by the compiler should be in this list.
    if {
        let __tmp_x = {
            let __tmp_x = {
                let __tmp_x = {
                    let __tmp_x = {
                        let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = internal_goarch::IS_AMD64; let __tmp_y = internal_goarch::IS_ARM64; __tmp_x | __tmp_y }; let __tmp_y = internal_goarch::IS_PPC64; __tmp_x | __tmp_y }; let __tmp_y = internal_goarch::IS_PPC64LE; __tmp_x | __tmp_y }; let __tmp_y = internal_goarch::IS_MIPS64; __tmp_x | __tmp_y };
                        let __tmp_y = internal_goarch::IS_MIPS64LE;
                        __tmp_x | __tmp_y
                    };
                    let __tmp_y = internal_goarch::IS_S390X;
                    __tmp_x | __tmp_y
                };
                let __tmp_y = internal_goarch::IS_RISCV64;
                __tmp_x | __tmp_y
            };
            let __tmp_y = internal_goarch::IS_LOONG64;
            __tmp_x | __tmp_y
        };
        let __tmp_y = 1;
        __tmp_x == __tmp_y
    } {
        { let __target = (*mp.lock().unwrap().as_ref().unwrap()).cheaprand.clone(); let __rhs = 0xa0761d6478bd642f as u64; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        let (mut hi, mut lo) = internal_runtime_math::mul64(Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).cheaprand.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).cheaprand.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0xe7037ed1a0b428db as u64; __tmp_x ^ __tmp_y }))));
        return (*Arc::new(Mutex::new(Some(({ let __tmp_x = hi; let __tmp_y = lo; __tmp_x ^ __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap());
    }

        // Implement xorshift64+: 2 32-bit xorshift sequences added together.
        // Shift triplet [17,7,16] was calculated as indicated in Marsaglia's
        // Xorshift paper: https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf
        // This generator passes the SmallCrush suite, part of TestU01 framework:
        // http://simul.iro.umontreal.ca/testu01/tu01.html
    let mut t: GoPtr<[u32; 2]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*mp.lock().unwrap().as_ref().unwrap()).cheaprand.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let (mut s1, mut s0) = (Arc::new(Mutex::new(Some({ let __seq = t.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() }))), Arc::new(Mutex::new(Some({ let __seq = t.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() }))));
    { let __rhs = { let __tmp_x = { let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 17; __tmp_x << __tmp_y }; let mut guard = s1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
    { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*s0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x ^ __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7; __tmp_x >> __tmp_y }; __tmp_x ^ __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*s0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }; __tmp_x ^ __tmp_y }; *s1.lock().unwrap() = Some(new_val); };
    {
        let __tmp_0 = (*s0.lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_1 = (*s1.lock().unwrap().as_ref().unwrap()).clone();
        t.with_mut(|__seq| { __seq[(0) as usize] = __tmp_0; });
        t.with_mut(|__seq| { __seq[(1) as usize] = __tmp_1; });
    };
    return { let __tmp_x = { let __v = (*s0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
}

/// cheaprand64 is a non-cryptographic-quality 63-bit random generator
/// suitable for calling at very high frequency (such as during sampling decisions).
/// it is "cheap" in the sense of both expense and quality.
///
/// cheaprand64 must not be exported to other packages:
/// the rule is that other packages using runtime-provided
/// randomness must always use rand.
///
/// cheaprand64 should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/zhangyunhao116/fastrand
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname cheaprand64
///go:nosplit
pub fn cheaprand64() -> i64 {
    return {
        let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 31; __tmp_x << __tmp_y };
        let __tmp_y = (*Arc::new(Mutex::new(Some(cheaprand() as i64))).lock().unwrap().as_ref().unwrap());
        __tmp_x ^ __tmp_y
    };
}

/// cheaprandn is like cheaprand() % n but faster.
///
/// cheaprandn must not be exported to other packages:
/// the rule is that other packages using runtime-provided
/// randomness must always use randn.
///
/// cheaprandn should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/phuslu/log
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname cheaprandn
///go:nosplit
pub fn cheaprandn(n: Arc<Mutex<Option<u32>>>) -> u32 {
        // See https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }); let __tmp_y = 32; __tmp_x >> __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap())
}

#[derive(Clone)]
pub struct AnonymousStruct24 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub seed: Arc<Mutex<Option<[u8; 32]>>>,
    pub state: Arc<Mutex<Option<internal_chacha8rand::chacha8::State>>>,
    pub init: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct24 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.seed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.init.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            seed: __go_clone_1_0,
            state: __go_clone_2_0,
            init: __go_clone_3_0,
        }
    }
}


impl Default for AnonymousStruct24 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            lock: __go_default_0_0,
            seed: __go_default_1_0,
            state: __go_default_2_0,
            init: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.seed));
        let __go_fmt_2 = format!("{}", (*self.state.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.init.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for AnonymousStruct24 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type globalRand = AnonymousStruct24;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
