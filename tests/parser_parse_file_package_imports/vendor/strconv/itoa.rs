use go2rust_stdlib_stubs::*;

use crate::{GoInteger, go_integer_add_one, go_integer_cast, go_integer_from_i128, go_integer_sub_one};

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::atoi::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::quote::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

pub(crate) const FAST_SMALLS: bool = true;


pub(crate) const N_SMALLS: i32 = 100;


pub(crate) const SMALLS_STRING: &'static str = "00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";


pub(crate) const HOST32BIT: bool = (!(((0 as u64) as u64)) >> (32 as u64)) as u64 == 0 as u64;


pub(crate) const DIGITS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";


/// FormatInt returns the string representation of i in the given base,
/// for 2 <= base <= 36. The result uses the lower-case letters 'a' to 'z'
/// for digit values >= 10.
pub fn format_int(i: Arc<Mutex<Option<i64>>>, base: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    if FAST_SMALLS && { let __tmp_x = 0 as i64; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = N_SMALLS as i64; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x == __tmp_y } {
        return small(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as i32))));
    }
    let (_, mut s) = format_bits(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y }))), Arc::new(Mutex::new(Some(false))));
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// Itoa is equivalent to [FormatInt](int64(i), 10).
pub fn itoa(i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    format_int(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as i64))), Arc::new(Mutex::new(Some(10))))
}

/// small returns the string for an i with 0 <= i < nSmalls.
pub fn small(i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __s = &(DIGITS); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() })));
    }
    Arc::new(Mutex::new(Some({ let __s = &(SMALLS_STRING); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }) as usize; let __high = ({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() })))
}

/// formatBits computes the string representation of u in the given base.
/// If neg is set, u is treated as negative int64 value. If append_ is
/// set, the string is appended to dst and the resulting byte slice is
/// returned as the first result value; otherwise the string is returned
/// as the second result value.
pub fn format_bits(dst: Arc<Mutex<Option<Vec<u8>>>>, mut u: Arc<Mutex<Option<u64>>>, base: Arc<Mutex<Option<i32>>>, neg: Arc<Mutex<Option<bool>>>, append_: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<String>>>) {
    let mut d: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } || { let __tmp_x = ({ let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 36; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("strconv: illegal AppendInt/FormatInt base".to_string()) as Box<dyn Any + Send + Sync>);
    }

        // 2 <= base && base <= len(digits)
    let mut a: Arc<Mutex<Option<[u8; 65]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut i = Arc::new(Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()).len() as i32)));

    if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = ((*u.lock().unwrap().as_ref().unwrap())).wrapping_neg(); *u.lock().unwrap() = Some(new_val); };
    }

        // convert bits
        // We use uint values where we can because those will
        // fit into a single register even on a 32bit machine.
    if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x == __tmp_y } {
                // common case: use constants for / because
                // the compiler can optimize it into a multiply+shift
        if HOST32BIT {
                // convert the lower digits using 32bit operations
        while { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as u64; __tmp_x >= __tmp_y } {
                // Avoid using r = a%b in addition to q = a/b
                // since 64bit division and modulo operations
                // are calculated by runtime functions on 32bit machines.
        let mut q = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as u64; __tmp_x / __tmp_y })));
        let mut us = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as u64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y }) as u64)));
        let mut j = Arc::new(Mutex::new(Some(4)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut is = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*us.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100 as u64; __tmp_x % __tmp_y }; let __tmp_y = 2 as u64; __tmp_x * __tmp_y })));
        { let __rhs = 100 as u64; let mut guard = us.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
        { let __rhs = 2; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        (*a.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] = { let __s = &(SMALLS_STRING); __s.as_bytes()[({ let __tmp_x = { let __v = (*is.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }) as usize] };
        (*a.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x + __tmp_y }) as usize] = { let __s = &(SMALLS_STRING); __s.as_bytes()[({ let __tmp_x = { let __v = (*is.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x + __tmp_y }) as usize] };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

                // us < 10, since it contains the last digit
                // from the initial 9-digit us.
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(SMALLS_STRING); __s.as_bytes()[({ let __tmp_x = { let __tmp_x = { let __v = (*us.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u64; __tmp_x * __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }) as usize] };

        { let new_val = q.lock().unwrap().as_ref().unwrap().clone(); *u.lock().unwrap() = Some(new_val); };
    }
    }
                // convert the lower digits using 32bit operations
                // Avoid using r = a%b in addition to q = a/b
                // since 64bit division and modulo operations
                // are calculated by runtime functions on 32bit machines.
                // u % 1e9 fits into a uint
                // us < 10, since it contains the last digit
                // from the initial 9-digit us.
                // u < 1e9
                // u guaranteed to fit into a uint
        let mut us = Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u64)));
        while { let __tmp_x = { let __v = (*us.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100 as u64; __tmp_x >= __tmp_y } {
        let mut is = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*us.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100 as u64; __tmp_x % __tmp_y }; let __tmp_y = 2 as u64; __tmp_x * __tmp_y })));
        { let __rhs = 100 as u64; let mut guard = us.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
        { let __rhs = 2; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        (*a.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] = { let __s = &(SMALLS_STRING); __s.as_bytes()[({ let __tmp_x = { let __v = (*is.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }) as usize] };
        (*a.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x + __tmp_y }) as usize] = { let __s = &(SMALLS_STRING); __s.as_bytes()[({ let __tmp_x = { let __v = (*is.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x + __tmp_y }) as usize] };
    }
                // us < 100
        let mut is = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*us.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u64; __tmp_x * __tmp_y })));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(SMALLS_STRING); __s.as_bytes()[({ let __tmp_x = { let __v = (*is.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }) as usize] };
        if { let __tmp_x = { let __v = (*us.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x >= __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(SMALLS_STRING); __s.as_bytes()[({ let __v = (*is.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] };
    }
    } else if is_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        let mut shift = Arc::new(Mutex::new(Some(bits::trailing_zeros(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)))) as u64)));
        let mut b = Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)));
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));
        while { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[({ let __tmp_x = (*Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }) as usize] };
        { let __rhs = (*shift.lock().unwrap().as_ref().unwrap()); let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[(*Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()) as usize] };
    } else {
        let mut b = Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)));
        while { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }

        let mut q = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y })));
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[(*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()) as usize] };
        { let new_val = q.lock().unwrap().as_ref().unwrap().clone(); *u.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[(*Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()) as usize] };
    }

        // common case: use constants for / because
        // the compiler can optimize it into a multiply+shift
        // convert the lower digits using 32bit operations
        // Avoid using r = a%b in addition to q = a/b
        // since 64bit division and modulo operations
        // are calculated by runtime functions on 32bit machines.
        // u % 1e9 fits into a uint
        // us < 10, since it contains the last digit
        // from the initial 9-digit us.
        // u < 1e9
        // u guaranteed to fit into a uint
        // us < 100
        // Use shifts and masks instead of / and %.
        // == 1<<shift - 1
        // u < base
        // general case
        // Avoid using r = a%b in addition to q = a/b
        // since 64bit division and modulo operations
        // are calculated by runtime functions on 32bit machines.
        // u < base
        // add sign, if any
    if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('-' as i32) as u8;
    }

    if { let __v = (*append_.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = { let __append_target = dst.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; d = new_val; };
        return (d.clone(), s.clone());
    }
    { let new_val = Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    (d.clone(), s.clone())
}

pub fn is_power_of_two(x: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y };
}