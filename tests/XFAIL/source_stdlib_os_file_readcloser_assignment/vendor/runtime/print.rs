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

use crate::{
    lock_spinbit::{lock, unlock},
    panic::{panicking},
    runtime2::{g, m, mutex},
    stubs::{getg},
    symtab::{findfunc, funcInfo, funcname},
    write_err::{write_err},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// The compiler knows that a print of a value of this type
/// should use printhex instead of printuint (decimal).
#[derive(Debug, Clone, Default)]
pub struct hex(pub Arc<Mutex<Option<u64>>>);

impl Display for hex {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for hex {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for hex {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for hex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for hex {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<hex> for u64 {
    fn eq(&self, other: &hex) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<hex> for u64 {
    fn partial_cmp(&self, other: &hex) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for hex {
    type Output = hex;
    fn add(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for hex {
    type Output = hex;
    fn add(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<hex> for u64 {
    type Output = hex;
    fn add(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for hex {
    type Output = hex;
    fn sub(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for hex {
    type Output = hex;
    fn sub(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<hex> for u64 {
    type Output = hex;
    fn sub(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for hex {
    type Output = hex;
    fn mul(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for hex {
    type Output = hex;
    fn mul(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<hex> for u64 {
    type Output = hex;
    fn mul(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for hex {
    type Output = hex;
    fn div(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for hex {
    type Output = hex;
    fn div(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<hex> for u64 {
    type Output = hex;
    fn div(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for hex {
    type Output = hex;
    fn rem(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for hex {
    type Output = hex;
    fn rem(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<hex> for u64 {
    type Output = hex;
    fn rem(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for hex {
    type Output = hex;
    fn bitand(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for hex {
    type Output = hex;
    fn bitand(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<hex> for u64 {
    type Output = hex;
    fn bitand(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for hex {
    type Output = hex;
    fn bitor(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for hex {
    type Output = hex;
    fn bitor(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<hex> for u64 {
    type Output = hex;
    fn bitor(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for hex {
    type Output = hex;
    fn bitxor(self, other: Self) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for hex {
    type Output = hex;
    fn bitxor(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<hex> for u64 {
    type Output = hex;
    fn bitxor(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for hex {
    type Output = hex;
    fn not(self) -> hex {
        hex(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for hex {
    type Output = hex;
    fn shl(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for hex {
    type Output = hex;
    fn shl(self, other: i32) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for hex {
    type Output = hex;
    fn shl(self, other: i8) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for hex {
    type Output = hex;
    fn shl(self, other: i16) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for hex {
    type Output = hex;
    fn shl(self, other: i64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for hex {
    type Output = hex;
    fn shl(self, other: u32) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for hex {
    type Output = hex;
    fn shl(self, other: u8) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for hex {
    type Output = hex;
    fn shl(self, other: u16) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for hex {
    type Output = hex;
    fn shl(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for hex {
    type Output = hex;
    fn shl(self, other: usize) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for hex {
    type Output = hex;
    fn shr(self, other: hex) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for hex {
    type Output = hex;
    fn shr(self, other: i32) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for hex {
    type Output = hex;
    fn shr(self, other: i8) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for hex {
    type Output = hex;
    fn shr(self, other: i16) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for hex {
    type Output = hex;
    fn shr(self, other: i64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for hex {
    type Output = hex;
    fn shr(self, other: u32) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for hex {
    type Output = hex;
    fn shr(self, other: u8) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for hex {
    type Output = hex;
    fn shr(self, other: u16) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for hex {
    type Output = hex;
    fn shr(self, other: u64) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for hex {
    type Output = hex;
    fn shr(self, other: usize) -> hex {
        hex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for hex {}

impl Ord for hex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static printBacklog: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 512]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static printBacklogIndex: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static debuglock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static minhexdigits: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *printBacklog.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *printBacklogIndex.lock().unwrap() = Some(0);
    *debuglock.lock().unwrap() = Some(Default::default());
    *minhexdigits.lock().unwrap() = Some(0);
    *minhexdigits.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *printBacklog.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *printBacklogIndex.lock().unwrap() = Some(0);
    *debuglock.lock().unwrap() = Some(Default::default());
    *minhexdigits.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_55() {
    *minhexdigits.lock().unwrap() = Some(0);
}


/// recordForPanic maintains a circular buffer of messages written by the
/// runtime leading up to a process crash, allowing the messages to be
/// extracted from a core dump.
///
/// The text written during a process crash (following "panic" or "fatal
/// error") is not saved, since the goroutine stacks will generally be readable
/// from the runtime data structures in the core file.
pub fn record_for_panic(b: Arc<Mutex<Option<Vec<u8>>>>) {
    printlock();

    if { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Not actively crashing: maintain circular buffer of print output.
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        let mut n = {
            let _dst_start = ((*printBacklogIndex.lock().unwrap().as_ref().unwrap())) as usize;
            let _dst_len = (*printBacklog.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*printBacklog.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        };
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = printBacklogIndex.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = 512; let mut guard = printBacklogIndex.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() % __rhs); };
    }
    }

        // Not actively crashing: maintain circular buffer of print output.
    printunlock();
}

pub fn printlock() {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).printlock.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).printlock.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i8; __tmp_x == __tmp_y } {
        lock(GoPtr::local(debuglock.clone()));
    }
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
}

pub fn printunlock() {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).printlock.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).printlock.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i8; __tmp_x == __tmp_y } {
        unlock(GoPtr::local(debuglock.clone()));
    }
}

/// write to goroutine-local buffer if diverting output,
/// or else standard error.
pub fn gwrite(b: Arc<Mutex<Option<Vec<u8>>>>) {
    if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return;
    }
    record_for_panic(b.clone());
    let mut gp = getg();

        // Don't use the writebuf if gp.m is dying. We want anything
        // written through gwrite to appear in the terminal rather
        // than be written to in some buffer, if we're in a panicking state.
        // Note that we can't just clear writebuf in the gp.m.dying case
        // because a panic isn't allowed to have any write barriers.
    if { let __nil_result = (*gp.lock().unwrap()).is_none(); __nil_result } || { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).writebuf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } || { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).dying.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        write_err(b.clone());
        return;
    }

    let mut n = {
        let _dst_start = (({ let __len_target = { let __field = (*gp.lock().unwrap().as_ref().unwrap()).writebuf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize;
        let _dst_len = ((({ let __cap_target = { let __field = (*gp.lock().unwrap().as_ref().unwrap()).writebuf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) })) as usize) - _dst_start;
        let _src = { let __copy_src_holder = b.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*(*gp.lock().unwrap().as_ref().unwrap()).writebuf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i32)))
    };
    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = (*gp.lock().unwrap().as_ref().unwrap()).writebuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = (*gp.lock().unwrap().as_ref().unwrap()).writebuf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); (*gp.lock().unwrap().as_mut().unwrap()).writebuf = new_val; };
}

/// hexdumpWords prints a word-oriented hex dump of [p, end).
///
/// If mark != nil, it will be called with each printed word's address
/// and should return a character mark to appear just before that
/// word's value. It can return 0 to indicate no mark.
pub fn hexdump_words(p: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>, mark: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> u8 + Send + Sync>>>>) {
    printlock();
    let mut markbuf: Arc<Mutex<Option<[u8; 1]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    (*markbuf.lock().unwrap().as_mut().unwrap())[(0) as usize] = (' ' as i32) as u8;
    { let new_val = Arc::new(Mutex::new(Some(((std::mem::size_of::<usize>() as i32) * (2 as i32)) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *minhexdigits.lock().unwrap() = __moved_val; };
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16 as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprintln!();
    }
        {
            let __go_print_arg_0 = format!("{}", hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as u64)))));
            let __go_print_arg_1 = format!("{}", ": ".to_string());
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }

        if { let __nil_result = (*mark.lock().unwrap()).is_some(); __nil_result } {
        (*markbuf.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> u8 + Send + Sync> = { let mut __f_guard = mark.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))) };
        if { let __tmp_x = { let __seq = { let __seq_holder = markbuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        (*markbuf.lock().unwrap().as_mut().unwrap())[(0) as usize] = (' ' as i32) as u8;
    }
    }
        gwrite(Arc::new(Mutex::new(Some({ let __seq_holder = markbuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        let mut val = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        {
            let __go_print_arg_0 = format!("{}", hex(Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()) as u64)))));
            eprint!("{}", __go_print_arg_0)
        };
        {
            let __go_print_arg_0 = format!("{}", " ".to_string());
            eprint!("{}", __go_print_arg_0)
        };

                // Can we symbolize val?
        let mut r#fn = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*r#fn.lock().unwrap().as_ref().unwrap()).valid() {
        {
            let __go_print_arg_0 = format!("{}", "<".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "+".to_string());
            let __go_print_arg_3 = format!("{}", hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*r#fn.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x - __tmp_y } as u64)))));
            let __go_print_arg_4 = format!("{}", "> ".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
    }
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // Can we symbolize val?
    { let new_val = 0; *minhexdigits.lock().unwrap() = Some(new_val); };
    eprintln!();
    printunlock();
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
