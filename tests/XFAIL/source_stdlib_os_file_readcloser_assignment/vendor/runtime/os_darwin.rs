use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{cgo::{iscgo}, defs_darwin_arm64::{__E_T_I_M_E_D_O_U_T, __P_T_H_R_E_A_D__C_R_E_A_T_E__D_E_T_A_C_H_E_D, __S_A__O_N_S_T_A_C_K, __S_A__R_E_S_T_A_R_T, __S_A__S_I_G_I_N_F_O, pthread, pthreadattr, pthreadcond, pthreadmutex, stackt, timespec, usigactiont}, malloc::{physPageSize}, note_other::{note}, panic::{throw}, proc::{FAILTHREADCREATE, malg}, r#extern::{G_O_A_R_C_H, G_O_O_S}, r#mod::{write_err_str}, retry::{retry_on_e_a_g_a_i_n}, runtime2::{g, m, stack}, signal_darwin_arm64::{sigctxt}, signal_unix::{minit_signal_mask, minit_signal_stack, set_thread_c_p_u_profiler_hz, sighandler, unminit_signals}, stubs::{getg}, sys_darwin::{exit, mlock, pthread_attr_getstacksize, pthread_attr_init, pthread_attr_setdetachstate, pthread_cond_init, pthread_cond_signal, pthread_cond_timedwait_relative_np, pthread_cond_wait, pthread_create, pthread_kill, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, pthread_self, sigaction, sigprocmask, usleep, usleep_no_g}, time_nofake::{nanotime, write}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __C_T_L__H_W: i32 = 6;
pub(crate) const __H_W__N_C_P_U: i32 = 3;
pub(crate) const __H_W__P_A_G_E_S_I_Z_E: i32 = 7;


pub(crate) const __N_S_I_G: i32 = 32;
pub(crate) const __S_I__U_S_E_R: i32 = 0;
pub(crate) const __S_I_G__B_L_O_C_K: i32 = 1;
pub(crate) const __S_I_G__U_N_B_L_O_C_K: i32 = 2;
pub(crate) const __S_I_G__S_E_T_M_A_S_K: i32 = 3;
pub(crate) const __S_S__D_I_S_A_B_L_E: i32 = 4;


pub(crate) const SIG_PER_THREAD_SYSCALL: i64 = 1 << 31;


#[derive(Debug, Clone)]
pub struct mOS {
    pub initialized: Arc<Mutex<Option<bool>>>,
    pub mutex: Arc<Mutex<Option<pthreadmutex>>>,
    pub cond: Arc<Mutex<Option<pthreadcond>>>,
    pub count: Arc<Mutex<Option<i32>>>,
}

impl mOS {
    pub fn __go_value_clone(&self) -> Self {
        Self { initialized: { let __guard = self.initialized.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mutex: { let __guard = self.mutex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cond: { let __guard = self.cond.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, count: { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mOS {
    fn default() -> Self {
        Self { initialized: Arc::new(Mutex::new(Some(false))), mutex: Arc::new(Mutex::new(Some(pthreadmutex::default()))), cond: Arc::new(Mutex::new(Some(pthreadcond::default()))), count: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for mOS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.initialized.lock().unwrap().as_ref().unwrap()), (*self.mutex.lock().unwrap().as_ref().unwrap()), (*self.cond.lock().unwrap().as_ref().unwrap()), (*self.count.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mOS {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct sigset(pub Arc<Mutex<Option<u32>>>);

impl Display for sigset {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for sigset {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for sigset {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for sigset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for sigset {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<sigset> for u32 {
    fn eq(&self, other: &sigset) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<sigset> for u32 {
    fn partial_cmp(&self, other: &sigset) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for sigset {
    type Output = sigset;
    fn add(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for sigset {
    type Output = sigset;
    fn add(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<sigset> for u32 {
    type Output = sigset;
    fn add(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for sigset {
    type Output = sigset;
    fn sub(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for sigset {
    type Output = sigset;
    fn sub(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<sigset> for u32 {
    type Output = sigset;
    fn sub(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for sigset {
    type Output = sigset;
    fn mul(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for sigset {
    type Output = sigset;
    fn mul(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<sigset> for u32 {
    type Output = sigset;
    fn mul(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for sigset {
    type Output = sigset;
    fn div(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for sigset {
    type Output = sigset;
    fn div(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<sigset> for u32 {
    type Output = sigset;
    fn div(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for sigset {
    type Output = sigset;
    fn rem(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for sigset {
    type Output = sigset;
    fn rem(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<sigset> for u32 {
    type Output = sigset;
    fn rem(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for sigset {
    type Output = sigset;
    fn bitand(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for sigset {
    type Output = sigset;
    fn bitand(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<sigset> for u32 {
    type Output = sigset;
    fn bitand(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for sigset {
    type Output = sigset;
    fn bitor(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for sigset {
    type Output = sigset;
    fn bitor(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<sigset> for u32 {
    type Output = sigset;
    fn bitor(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for sigset {
    type Output = sigset;
    fn bitxor(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for sigset {
    type Output = sigset;
    fn bitxor(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<sigset> for u32 {
    type Output = sigset;
    fn bitxor(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for sigset {
    type Output = sigset;
    fn not(self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for sigset {
    type Output = sigset;
    fn shl(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for sigset {
    type Output = sigset;
    fn shl(self, other: i32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for sigset {
    type Output = sigset;
    fn shl(self, other: i8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for sigset {
    type Output = sigset;
    fn shl(self, other: i16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for sigset {
    type Output = sigset;
    fn shl(self, other: i64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for sigset {
    type Output = sigset;
    fn shl(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for sigset {
    type Output = sigset;
    fn shl(self, other: u8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for sigset {
    type Output = sigset;
    fn shl(self, other: u16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for sigset {
    type Output = sigset;
    fn shl(self, other: u64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for sigset {
    type Output = sigset;
    fn shl(self, other: usize) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for sigset {
    type Output = sigset;
    fn shr(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for sigset {
    type Output = sigset;
    fn shr(self, other: i32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for sigset {
    type Output = sigset;
    fn shr(self, other: i8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for sigset {
    type Output = sigset;
    fn shr(self, other: i16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for sigset {
    type Output = sigset;
    fn shr(self, other: i64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for sigset {
    type Output = sigset;
    fn shr(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for sigset {
    type Output = sigset;
    fn shr(self, other: u8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for sigset {
    type Output = sigset;
    fn shr(self, other: u16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for sigset {
    type Output = sigset;
    fn shr(self, other: u64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for sigset {
    type Output = sigset;
    fn shr(self, other: usize) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for sigset {}

impl Ord for sigset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static sigNoteRead: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigNoteWrite: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigset_all: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sigset>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static executablePath: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *sigNoteRead.lock().unwrap() = Some(0);
    *sigNoteWrite.lock().unwrap() = Some(0);
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(0)))));
    *executablePath.lock().unwrap() = Some(String::new());
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(!0 as u32)))));
}


pub(crate) fn __go_zero_globals() {
    *sigNoteRead.lock().unwrap() = Some(0);
    *sigNoteWrite.lock().unwrap() = Some(0);
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(0)))));
    *executablePath.lock().unwrap() = Some(String::new());
}


pub(crate) fn __go_init_order_42() {
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(!0 as u32)))));
}


///go:nosplit
pub fn semacreate(mp: Arc<Mutex<Option<m>>>) {
    if (*(*(*mp.lock().unwrap().as_ref().unwrap()).m_o_s.lock().unwrap().as_ref().unwrap()).initialized.lock().unwrap().as_ref().unwrap()) {
        return;
    }
    { let new_val = true; *(*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).initialized.lock().unwrap() = Some(new_val); };
    {
        let mut err = pthread_mutex_init((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone(), Arc::new(Mutex::new(None)));;
        if { let __tmp_x = err; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
            throw(Arc::new(Mutex::new(Some("pthread_mutex_init".to_string()))));;
        }
    }
    {
        let mut err = pthread_cond_init((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).cond.clone(), Arc::new(Mutex::new(None)));;
        if { let __tmp_x = err; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
            throw(Arc::new(Mutex::new(Some("pthread_cond_init".to_string()))));;
        }
    }
}

///go:nosplit
pub fn semasleep(ns: Arc<Mutex<Option<i64>>>) -> i32 {
    let mut start: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        { let new_val = nanotime(); *start.lock().unwrap() = Some(new_val); };
    }
    let mut g = getg();
    let mut mp = (*g.lock().unwrap().as_ref().unwrap()).m.clone();
    if { let __left = g.clone(); let __right = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // sema sleep/wakeup are implemented with pthreads, which are not async-signal-safe on Darwin.
        throw(Arc::new(Mutex::new(Some("semasleep on Darwin signal stack".to_string()))));
    }
        // sema sleep/wakeup are implemented with pthreads, which are not async-signal-safe on Darwin.
    pthread_mutex_lock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
    loop {
        if { let __tmp_x = (*(*(*mp.lock().unwrap().as_ref().unwrap()).m_o_s.lock().unwrap().as_ref().unwrap()).count.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __target = (*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        pthread_mutex_unlock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
        return 0;
    }
        if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        let mut spent = Arc::new(Mutex::new(Some({ let __tmp_x = nanotime(); let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*spent.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        pthread_mutex_unlock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
        return -(1);
    }
        let mut t: Arc<Mutex<Option<timespec>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*t.lock().unwrap().as_mut().unwrap()).set_nsec(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*spent.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
        let mut err = pthread_cond_timedwait_relative_np((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).cond.clone(), (*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone(), t.clone());
        if { let __tmp_x = err; let __tmp_y = __E_T_I_M_E_D_O_U_T as i32; __tmp_x == __tmp_y } {
        pthread_mutex_unlock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
        return -(1);
    }
    } else {
        pthread_cond_wait((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).cond.clone(), (*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
    }
    }
}

///go:nosplit
pub fn semawakeup(mp: GoPtr<crate::runtime2::m>) {
    {
        let mut g = getg();;
        if { let __left = g.clone(); let __right = (*(*g.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
            throw(Arc::new(Mutex::new(Some("semawakeup on Darwin signal stack".to_string()))));;
        }
    }
    pthread_mutex_lock({ let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().mutex.clone(); __field }); __ptr_value }.clone());
    { let __target = { let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().count.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    if { let __tmp_x = (*{ let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().m_o_s.lock().unwrap().as_ref().unwrap().count.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        pthread_cond_signal({ let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().cond.clone(); __field }); __ptr_value }.clone());
    }
    pthread_mutex_unlock({ let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().mutex.clone(); __field }); __ptr_value }.clone());
}

/// sigNoteWakeup wakes up a thread sleeping on a note created by sigNoteSetup.
pub fn sig_note_wakeup(__arg0: Arc<Mutex<Option<note>>>) {
    let mut b: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
    write(Arc::new(Mutex::new(Some((*sigNoteWrite.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(Arc::as_ptr(&b.clone()) as usize))), Arc::new(Mutex::new(Some(1 as i32))));
}

/// May run with m.p==nil, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn newosproc(mp: GoPtr<crate::runtime2::m>) {
    let mut stk = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if false {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "newosproc stk=".to_string()), format!("{}", { let __v = (*stk.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " m=".to_string()), format!("{}", format!("0x{:x}", mp.addr())), format!("{}", " g=".to_string()), format!("{}", format!("&{}", (*{ let __field = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.clone(); __field }.lock().unwrap().as_ref().unwrap()))), format!("{}", " id=".to_string()), format!("{}", (*{ let __ptr_value = mp.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " ostk=".to_string()), format!("{}", format!("0x{:x}", &mp as *const _ as usize)), format!("{}", "\n".to_string()));
    }

        // Initialize an attribute object.
    let mut attr: Arc<Mutex<Option<pthreadattr>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut err: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let new_val = pthread_attr_init(attr.clone()); *err.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __v = (*err.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }

        // Find out OS stack size for our own stack guard.
    let mut stacksize: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = pthread_attr_getstacksize(attr.clone(), stacksize.clone()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }
    { let new_val = stacksize.lock().unwrap().as_ref().unwrap().clone(); *(*(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap() = Some(new_val); };

        // Tell the pthread library we won't join with this thread.
    if { let __tmp_x = pthread_attr_setdetachstate(attr.clone(), Arc::new(Mutex::new(Some(2)))); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }

        // Finally, create the thread. It starts at mstart_stub, which does some low-level
        // setup and then calls mstart.
    let mut oset: Arc<Mutex<Option<sigset>>> = Arc::new(Mutex::new(Some(sigset(Arc::new(Mutex::new(Some(0)))))));
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), sigset_all.clone(), oset.clone());
    let attr_closure_clone = attr.clone(); let mp_closure_clone = mp.clone(); { let new_val = retry_on_e_a_g_a_i_n(Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        return pthread_create(attr_closure_clone.clone(), Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(mstart_stub.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(mp.addr()))));
    }) as Box<dyn FnMut() -> i32 + Send + Sync>)))); *err.lock().unwrap() = Some(new_val); };
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), oset.clone(), Arc::new(Mutex::new(None)));
    if { let __tmp_x = { let __v = (*err.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }
}

/// glue code to call mstart from pthread_create.
pub fn mstart_stub() {
    unimplemented!("Go function declaration has no body");
}


/// Called to initialize a new m (including the bootstrap m).
/// Called on the parent thread (main thread in case of bootstrap), can allocate memory.
pub fn mpreinit(mp: Arc<Mutex<Option<m>>>) {
    { let new_val = malg(Arc::new(Mutex::new(Some({ let __tmp_x = 32; let __tmp_y = 1024; __tmp_x * __tmp_y } as i32)))).clone(); (*mp.lock().unwrap().as_mut().unwrap()).gsignal = new_val; };
    { let new_val = mp.clone(); (*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_mut().unwrap()).m = new_val; };
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y } {
                // mlock the signal stack to work around a kernel bug where it may
                // SIGILL when the signal stack is not faulted in while a signal
                // arrives. See issue 42774.
        mlock(Arc::new(Mutex::new(Some({ let __tmp_x = (*(*(*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

/// Called to initialize a new m (including the bootstrap m).
/// Called on the new thread, cannot allocate memory.
pub fn minit() {
        // iOS does not support alternate signal stack.
        // The signal handler handles it directly.
    if !({ let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y }) {
        minit_signal_stack();
    }
    minit_signal_mask();
    { let new_val = Arc::new(Mutex::new(Some((*(*pthread_self().lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).procid.lock().unwrap() = __moved_val; };
}

/// Called from dropm to undo the effect of an minit.
///
///go:nosplit
pub fn unminit() {
        // iOS does not support alternate signal stack.
        // See minit.
    if !({ let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y }) {
        unminit_signals();
    }
    { let new_val = 0 as u64; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).procid.lock().unwrap() = Some(new_val); };
}

///go:nosplit
pub fn osyield_no_g() {
    usleep_no_g(Arc::new(Mutex::new(Some(1 as u32))));
}

///go:nosplit
pub fn osyield() {
    usleep(Arc::new(Mutex::new(Some(1 as u32))));
}

///go:nosplit
///go:nowritebarrierrec
pub fn setsig(i: Arc<Mutex<Option<u32>>>, mut r#fn: Arc<Mutex<Option<usize>>>) {
    let mut sa: Arc<Mutex<Option<usigactiont>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = { let __tmp_x = { let __tmp_x = __S_A__S_I_G_I_N_F_O; let __tmp_y = __S_A__O_N_S_T_A_C_K; __tmp_x | __tmp_y }; let __tmp_y = __S_A__R_E_S_T_A_R_T; __tmp_x | __tmp_y } as i32; *(*sa.lock().unwrap().as_ref().unwrap()).sa_flags.lock().unwrap() = Some(new_val); };
    { let new_val = !(0 as u32) as u32; *(*sa.lock().unwrap().as_ref().unwrap()).sa_mask.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __v = (*r#fn.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(sighandler.clone()) as Box<dyn Any + Send + Sync>)))); __tmp_x == __tmp_y } {
        if (*iscgo.lock().unwrap().as_ref().unwrap()) {
        { let new_val = internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(cgo_sigtramp.clone()) as Box<dyn Any + Send + Sync>)))); *r#fn.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(sigtramp.clone()) as Box<dyn Any + Send + Sync>)))); *r#fn.lock().unwrap() = Some(new_val); };
    }
    }
    { unimplemented!("unsafe.Pointer dereference assignment"); };
    sigaction(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sa.clone(), Arc::new(Mutex::new(None)));
}

/// sigtramp is the callback from libc when a signal is received.
/// It is called with the C calling convention.
pub fn sigtramp() {
    unimplemented!("Go function declaration has no body");
}


pub fn cgo_sigtramp() {
    unimplemented!("Go function declaration has no body");
}


/// setSignalstackSP sets the ss_sp field of a stackt.
///
///go:nosplit
pub fn set_signalstack_s_p(s: Arc<Mutex<Option<stackt>>>, sp: Arc<Mutex<Option<usize>>>) {
    { unimplemented!("unsafe.Pointer dereference assignment"); };
}

///go:nosplit
///go:nowritebarrierrec
pub fn sigaddset(mask: Arc<Mutex<Option<sigset>>>, i: Arc<Mutex<Option<i32>>>) {
    { let __rhs = (*({ let __tmp_x = sigset(Arc::new(Mutex::new(Some(1 as u32)))); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
}

pub fn sigdelset(mask: Arc<Mutex<Option<sigset>>>, i: Arc<Mutex<Option<i32>>>) {
    { let __rhs = (*({ let __tmp_x = sigset(Arc::new(Mutex::new(Some(1 as u32)))); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
}

pub fn set_thread_c_p_u_profiler(hz: Arc<Mutex<Option<i32>>>) {
    set_thread_c_p_u_profiler_hz(Arc::new(Mutex::new(Some({ let __arg_holder = hz.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

///go:nosplit
pub fn valid_s_i_g_p_r_o_f(mp: Arc<Mutex<Option<m>>>, c: Arc<Mutex<Option<sigctxt>>>) -> bool {
    true
}

pub fn signal_m(mp: GoPtr<crate::runtime2::m>, sig_local: Arc<Mutex<Option<i32>>>) {
    pthread_kill(Arc::new(Mutex::new(Some(crate::defs_darwin_arm64::pthread(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.procid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))))))), Arc::new(Mutex::new(Some((*sig_local.lock().unwrap().as_ref().unwrap()) as u32))));
}

///go:nosplit
pub fn run_per_thread_syscall() {
    throw(Arc::new(Mutex::new(Some("runPerThreadSyscall only valid on linux".to_string()))));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for mOS {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
