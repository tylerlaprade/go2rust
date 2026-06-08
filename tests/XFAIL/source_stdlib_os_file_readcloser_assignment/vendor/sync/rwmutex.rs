use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped, go_any_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{mutex::{Locker, Mutex}, runtime::{fatal, runtime__semacquire_r_w_mutex, runtime__semacquire_r_w_mutex_r, runtime__semrelease}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

pub(crate) const RWMUTEX_MAX_READERS: i32 = 1 << 30;


/// A RWMutex is a reader/writer mutual exclusion lock.
/// The lock can be held by an arbitrary number of readers or a single writer.
/// The zero value for a RWMutex is an unlocked mutex.
///
/// A RWMutex must not be copied after first use.
///
/// If any goroutine calls [RWMutex.Lock] while the lock is already held by
/// one or more readers, concurrent calls to [RWMutex.RLock] will block until
/// the writer has acquired (and released) the lock, to ensure that
/// the lock eventually becomes available to the writer.
/// Note that this prohibits recursive read-locking.
/// A [RWMutex.RLock] cannot be upgraded into a [RWMutex.Lock],
/// nor can a [RWMutex.Lock] be downgraded into a [RWMutex.RLock].
///
/// In the terminology of [the Go memory model],
/// the n'th call to [RWMutex.Unlock] “synchronizes before” the m'th call to Lock
/// for any n < m, just as for [Mutex].
/// For any call to RLock, there exists an n such that
/// the n'th call to Unlock “synchronizes before” that call to RLock,
/// and the corresponding call to [RWMutex.RUnlock] “synchronizes before”
/// the n+1'th call to Lock.
///
/// [the Go memory model]: https://go.dev/ref/mem
#[derive(Clone)]
pub struct RWMutex {
    pub w: Arc<StdMutex<Option<Mutex>>>,
    pub writer_sem: Arc<StdMutex<Option<u32>>>,
    pub reader_sem: Arc<StdMutex<Option<u32>>>,
    pub reader_count: Arc<StdMutex<Option<sync_atomic::r#type::Int32>>>,
    pub reader_wait: Arc<StdMutex<Option<sync_atomic::r#type::Int32>>>,
}

impl RWMutex {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.writer_sem.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.reader_sem.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.reader_count.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.reader_wait.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            writer_sem: __go_clone_1_0,
            reader_sem: __go_clone_2_0,
            reader_count: __go_clone_3_0,
            reader_wait: __go_clone_4_0,
        }
    }
}


impl Default for RWMutex {
    fn default() -> Self {
        Self { w: Arc::new(StdMutex::new(Some(Mutex::default()))), writer_sem: Arc::new(StdMutex::new(Some(0))), reader_sem: Arc::new(StdMutex::new(Some(0))), reader_count: Arc::new(StdMutex::new(Some(Default::default()))), reader_wait: Arc::new(StdMutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for RWMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.writer_sem.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.reader_sem.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.reader_count.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.reader_wait.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for RWMutex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct rlocker {
    pub w: Arc<StdMutex<Option<Mutex>>>,
    pub writer_sem: Arc<StdMutex<Option<u32>>>,
    pub reader_sem: Arc<StdMutex<Option<u32>>>,
    pub reader_count: Arc<StdMutex<Option<sync_atomic::r#type::Int32>>>,
    pub reader_wait: Arc<StdMutex<Option<sync_atomic::r#type::Int32>>>,
}

impl rlocker {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.writer_sem.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.reader_sem.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.reader_count.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.reader_wait.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            writer_sem: __go_clone_1_0,
            reader_sem: __go_clone_2_0,
            reader_count: __go_clone_3_0,
            reader_wait: __go_clone_4_0,
        }
    }
}


impl Default for rlocker {
    fn default() -> Self {
        Self { w: Arc::new(StdMutex::new(Some(Mutex::default()))), writer_sem: Arc::new(StdMutex::new(Some(0))), reader_sem: Arc::new(StdMutex::new(Some(0))), reader_count: Arc::new(StdMutex::new(Some(Default::default()))), reader_wait: Arc::new(StdMutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for rlocker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.writer_sem.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.reader_sem.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.reader_count.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.reader_wait.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for rlocker {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl RWMutex {
    /// RLock locks rw for reading.
    ///
    /// It should not be used for recursive read locking; a blocked Lock
    /// call excludes new readers from acquiring the lock. See the
    /// documentation on the [RWMutex] type.
    pub fn r_lock(&self) {
        if internal_race::ENABLED {
        internal_race::read(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.w.clone()) as usize))));
        internal_race::disable();
    }
        if { let __tmp_x = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(StdMutex::new(Some(1 as i32)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
                // A writer is pending, wait for it.
        runtime__semacquire_r_w_mutex_r(self.reader_sem.clone(), Arc::new(StdMutex::new(Some(false))), Arc::new(StdMutex::new(Some(0))));
    }
                // A writer is pending, wait for it.
        if internal_race::ENABLED {
        internal_race::enable();
        internal_race::acquire(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.reader_sem.clone()) as usize))));
    }
    }

    /// TryRLock tries to lock rw for reading and reports whether it succeeded.
    ///
    /// Note that while correct uses of TryRLock do exist, they are rare,
    /// and use of TryRLock is often a sign of a deeper problem
    /// in a particular use of mutexes.
    pub fn try_r_lock(&self) -> bool {
        if internal_race::ENABLED {
        internal_race::read(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.w.clone()) as usize))));
        internal_race::disable();
    }
        loop {
        let mut c = (*self.reader_count.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = c; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        if internal_race::ENABLED {
        internal_race::enable();
    }
        return false;
    }
        if (*self.reader_count.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(StdMutex::new(Some(c))), Arc::new(StdMutex::new(Some({ let __tmp_x = c; let __tmp_y = 1 as i32; __tmp_x + __tmp_y })))) {
        if internal_race::ENABLED {
        internal_race::enable();
        internal_race::acquire(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.reader_sem.clone()) as usize))));
    }
        return true;
    }
    }
    }

    /// RUnlock undoes a single [RWMutex.RLock] call;
    /// it does not affect other simultaneous readers.
    /// It is a run-time error if rw is not locked for reading
    /// on entry to RUnlock.
    pub fn r_unlock(&self) {
        if internal_race::ENABLED {
        internal_race::read(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.w.clone()) as usize))));
        internal_race::release_merge(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.writer_sem.clone()) as usize))));
        internal_race::disable();
    }
        {
        let mut r = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(StdMutex::new(Some(-1 as i32))));;
        if { let __tmp_x = r; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
            self.r_unlock_slow(Arc::new(StdMutex::new(Some(r))));;
        }
    }
                // Outlined slow-path to allow the fast-path to be inlined
        if internal_race::ENABLED {
        internal_race::enable();
    }
    }

    pub fn r_unlock_slow(&self, r: Arc<StdMutex<Option<i32>>>) {
        if { let __tmp_x = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i32; __tmp_x + __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i32; __tmp_x + __tmp_y }; let __tmp_y = -RWMUTEX_MAX_READERS as i32; __tmp_x == __tmp_y } {
        internal_race::enable();
        fatal(Arc::new(StdMutex::new(Some("sync: RUnlock of unlocked RWMutex".to_string()))));
    }
                // A writer is pending.
        if { let __tmp_x = (*self.reader_wait.lock().unwrap().as_mut().unwrap()).add(Arc::new(StdMutex::new(Some(-1 as i32)))); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
                // The last reader unblocks the writer.
        runtime__semrelease(self.writer_sem.clone(), Arc::new(StdMutex::new(Some(false))), Arc::new(StdMutex::new(Some(1))));
    }
    }

    /// Lock locks rw for writing.
    /// If the lock is already locked for reading or writing,
    /// Lock blocks until the lock is available.
    pub fn lock(&self) {
        if internal_race::ENABLED {
        internal_race::read(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.w.clone()) as usize))));
        internal_race::disable();
    }
                // First, resolve competition with other writers.
        (*self.w.lock().unwrap().as_ref().unwrap()).lock();
                // Announce to readers there is a pending writer.
        let mut r = Arc::new(StdMutex::new(Some({ let __tmp_x = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(StdMutex::new(Some(-RWMUTEX_MAX_READERS as i32)))); let __tmp_y = RWMUTEX_MAX_READERS as i32; __tmp_x + __tmp_y })));
                // Wait for active readers.
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && { let __tmp_x = (*self.reader_wait.lock().unwrap().as_mut().unwrap()).add(Arc::new(StdMutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        runtime__semacquire_r_w_mutex(self.writer_sem.clone(), Arc::new(StdMutex::new(Some(false))), Arc::new(StdMutex::new(Some(0))));
    }
        if internal_race::ENABLED {
        internal_race::enable();
        internal_race::acquire(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.reader_sem.clone()) as usize))));
        internal_race::acquire(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.writer_sem.clone()) as usize))));
    }
    }

    /// TryLock tries to lock rw for writing and reports whether it succeeded.
    ///
    /// Note that while correct uses of TryLock do exist, they are rare,
    /// and use of TryLock is often a sign of a deeper problem
    /// in a particular use of mutexes.
    pub fn try_lock(&self) -> bool {
        if internal_race::ENABLED {
        internal_race::read(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.w.clone()) as usize))));
        internal_race::disable();
    }
        if !(*self.w.lock().unwrap().as_ref().unwrap()).try_lock() {
        if internal_race::ENABLED {
        internal_race::enable();
    }
        return false;
    }
        if !(*self.reader_count.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(StdMutex::new(Some(0 as i32))), Arc::new(StdMutex::new(Some(-RWMUTEX_MAX_READERS as i32)))) {
        (*self.w.lock().unwrap().as_ref().unwrap()).unlock();
        if internal_race::ENABLED {
        internal_race::enable();
    }
        return false;
    }
        if internal_race::ENABLED {
        internal_race::enable();
        internal_race::acquire(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.reader_sem.clone()) as usize))));
        internal_race::acquire(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.writer_sem.clone()) as usize))));
    }
        true
    }

    /// Unlock unlocks rw for writing. It is a run-time error if rw is
    /// not locked for writing on entry to Unlock.
    ///
    /// As with Mutexes, a locked [RWMutex] is not associated with a particular
    /// goroutine. One goroutine may [RWMutex.RLock] ([RWMutex.Lock]) a RWMutex and then
    /// arrange for another goroutine to [RWMutex.RUnlock] ([RWMutex.Unlock]) it.
    pub fn unlock(&self) {
        if internal_race::ENABLED {
        internal_race::read(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.w.clone()) as usize))));
        internal_race::release(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.reader_sem.clone()) as usize))));
        internal_race::disable();
    }
                // Announce to readers there is no active writer.
        let mut r = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(StdMutex::new(Some(RWMUTEX_MAX_READERS as i32))));
        if { let __tmp_x = r; let __tmp_y = RWMUTEX_MAX_READERS as i32; __tmp_x >= __tmp_y } {
        internal_race::enable();
        fatal(Arc::new(StdMutex::new(Some("sync: Unlock of unlocked RWMutex".to_string()))));
    }
                // Unblock blocked readers, if any.
        let mut i = Arc::new(StdMutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(StdMutex::new(Some(r as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        runtime__semrelease(self.reader_sem.clone(), Arc::new(StdMutex::new(Some(false))), Arc::new(StdMutex::new(Some(0))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Allow other writers to proceed.
        (*self.w.lock().unwrap().as_ref().unwrap()).unlock();
        if internal_race::ENABLED {
        internal_race::enable();
    }
    }

    /// RLocker returns a [Locker] interface that implements
    /// the [Locker.Lock] and [Locker.Unlock] methods by calling rw.RLock and rw.RUnlock.
    pub fn r_locker(&self) -> Arc<StdMutex<Option<Box<dyn Locker + Send + Sync>>>> {
        Arc::new(StdMutex::new(Some(Box::new((*Arc::new(StdMutex::new(Some(rlocker::default()))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Locker + Send + Sync>)))
    }
}

impl Locker for RWMutex {
    fn lock(&mut self) {
        RWMutex::lock(self)
    }
    fn unlock(&mut self) {
        RWMutex::unlock(self)
    }
    fn __go_clone_box_locker(&self) -> Box<dyn Locker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Locker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_locker(&self, other: &(dyn Locker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RWMutex>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RWMutexPtr(pub Arc<StdMutex<Option<RWMutex>>>);

impl std::fmt::Display for RWMutexPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Locker for RWMutexPtr {
    fn lock(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        RWMutex::lock(__recv)
    }
    fn unlock(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        RWMutex::unlock(__recv)
    }
    fn __go_clone_box_locker(&self) -> Box<dyn Locker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Locker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_locker(&self, other: &(dyn Locker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RWMutexPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl rlocker {
    pub fn lock(&self) {
        { let __recv = Arc::new(StdMutex::new(Some(RWMutex::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r_lock(); __result };
    }

    pub fn unlock(&self) {
        { let __recv = Arc::new(StdMutex::new(Some(RWMutex::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r_unlock(); __result };
    }
}

impl Locker for rlocker {
    fn lock(&mut self) {
        rlocker::lock(self)
    }
    fn unlock(&mut self) {
        rlocker::unlock(self)
    }
    fn __go_clone_box_locker(&self) -> Box<dyn Locker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Locker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_locker(&self, other: &(dyn Locker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rlocker>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct rlockerPtr(pub Arc<StdMutex<Option<rlocker>>>);

impl std::fmt::Display for rlockerPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Locker for rlockerPtr {
    fn lock(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        rlocker::lock(__recv)
    }
    fn unlock(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        rlocker::unlock(__recv)
    }
    fn __go_clone_box_locker(&self) -> Box<dyn Locker + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Locker + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_locker(&self, other: &(dyn Locker + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rlockerPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl GoValueClone for RWMutex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for rlocker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
