use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoByteSequence,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    format::{ParseError, R_F_C3339, STD_FRAC_SECOND9, append_int, append_nano, is_digit, parse, parse_nanoseconds, std_frac_second},
    r#mod::{Month, Time, absDays, absSeconds, date, days_in},
    zoneinfo::{Local, Location, UTC, fixed_zone},
};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::r#mod::Time {
    pub fn append_format_r_f_c3339(&self, mut b: Arc<Mutex<Option<Vec<u8>>>>, nanos: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        let (_, mut offset, mut abs) = self.locabs();
                // Format date.
        let (mut year, mut month, mut day) = crate::r#mod::absDays::date(&(*crate::r#mod::absSeconds::days(&(*abs.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some(year))), Arc::new(Mutex::new(Some(4)))); b = new_val; };
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some((*{ let __v = (*month.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some(day))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('T' as i32) as u8); __append_target.clone() }; b = new_val; };
                // Format time.
        let (mut hour, mut min, mut sec) = crate::r#mod::absSeconds::clock(&(*abs.lock().unwrap().as_ref().unwrap()));
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some(hour))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((':' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some(min))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((':' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some(sec))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        if { let __v = (*nanos.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut std = std_frac_second(Arc::new(Mutex::new(Some(35))), Arc::new(Mutex::new(Some(9))), Arc::new(Mutex::new(Some(('.' as i32) as i32))));
        { let new_val = append_nano(b.clone(), Arc::new(Mutex::new(Some(self.nanosecond()))), Arc::new(Mutex::new(Some(std)))); b = new_val; };
    }
        if { let __tmp_x = offset; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('Z' as i32) as u8); __append_target.clone() };
    }
                // Format zone.
        let mut zone = Arc::new(Mutex::new(Some({ let __tmp_x = offset; let __tmp_y = 60; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = -((*zone.lock().unwrap().as_ref().unwrap())); *zone.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('+' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((':' as i32) as u8); __append_target.clone() }; b = new_val; };
        { let new_val = append_int(b.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*zone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x % __tmp_y }))), Arc::new(Mutex::new(Some(2)))); b = new_val; };
        return b.clone();
    }

    pub fn append_strict_r_f_c3339(&self, mut b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut n0 = Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        { let new_val = self.append_format_r_f_c3339(b.clone(), Arc::new(Mutex::new(Some(true)))); b = new_val; };
                // Not all valid Go timestamps can be serialized as valid RFC 3339.
                // Explicitly check for these edge cases.
                // See https://go.dev/issue/4556 and https://go.dev/issue/54580.
        let mut num2 = Arc::new(Mutex::new(Some(Box::new(move |b: Arc<Mutex<Option<Vec<u8>>>>| -> u8 {
        return {
            let __go_binary_0 = 10 as u8;
            let __go_binary_1 = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
            let __go_binary_2 = ('0' as i32) as u8;
            let __go_binary_3 = __go_binary_1 - __go_binary_2;
            let __go_binary_4 = __go_binary_0 * __go_binary_3;
            let __go_binary_5 = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() };
            let __go_binary_6 = ('0' as i32) as u8;
            let __go_binary_7 = __go_binary_5 - __go_binary_6;
            let __go_binary_8 = __go_binary_4 + __go_binary_7;
            __go_binary_8
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync>)));
        if { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ({ let __v = (*n0.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 4; __tmp_x + __tmp_y }) as usize].clone() }; let __tmp_y = ('-' as i32) as u8; __tmp_x != __tmp_y } {
            return (b.clone(), errors::new(Arc::new(Mutex::new(Some("year outside of range [0,9999]".to_string())))));
        } else if { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('Z' as i32) as u8; __tmp_x != __tmp_y } {
            let mut c = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 6; __tmp_x - __tmp_y }) as usize].clone() })));
            if ({ let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y }) || { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync> = { let mut __f_guard = num2.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 5; __tmp_x - __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))) }; let __tmp_y = 24 as u8; __tmp_x >= __tmp_y } {
        return (b.clone(), errors::new(Arc::new(Mutex::new(Some("timezone hour outside of range [0,23]".to_string())))));
    }
        }
                // year must be exactly 4 digits wide
        return (b.clone(), Arc::new(Mutex::new(None)));
    }
}

pub fn parse_r_f_c3339<bytes: GoByteSequence + Clone + Send + Sync + 'static>(mut s: Arc<Mutex<Option<bytes>>>, local: Arc<Mutex<Option<Location>>>) -> (Arc<Mutex<Option<crate::r#mod::Time>>>, bool) {
        // parseUint parses s as an unsigned decimal integer and
        // verifies that it is within some range.
        // If it is invalid or out-of-range,
        // it sets ok to false and returns the min value.
    let mut ok = Arc::new(Mutex::new(Some(true)));
    let mut ok_closure_clone = ok.clone(); let mut parseUint = Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<bytes>>>, min: Arc<Mutex<Option<i32>>>, max: Arc<Mutex<Option<i32>>>| -> (i32) {
    let mut x: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        { let __range_holder = Arc::new(Mutex::new(Some(((*s.lock().unwrap().as_ref().unwrap())).as_bytes().to_vec()))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter().copied() {
        if { let __tmp_x = c; let __tmp_y = ('0' as i32) as u8; __tmp_x < __tmp_y } || { let __tmp_x = ('9' as i32) as u8; let __tmp_y = c; __tmp_x < __tmp_y } {
        { let new_val = false; *ok_closure_clone.lock().unwrap() = Some(new_val); };
        return { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(c as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = ('0' as i32); __tmp_x - __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    } }
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = false; *ok_closure_clone.lock().unwrap() = Some(new_val); };
        return { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync>)));

        // Parse the date and time.
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); let __tmp_y = 19; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), false);
    }
    let mut year = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((0) as usize, Some((4) as usize))))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(9999)))) };
    let mut month = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((5) as usize, Some((7) as usize))))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(12)))) };
    let mut day = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((8) as usize, Some((10) as usize))))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(days_in(Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some(month as i32))))))), Arc::new(Mutex::new(Some(year)))))))) };
    let mut hour = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((11) as usize, Some((13) as usize))))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(23)))) };
    let mut min = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((14) as usize, Some((16) as usize))))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(59)))) };
    let mut sec = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((17) as usize, Some((19) as usize))))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(59)))) };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || !({ let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((4) as usize); let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((7) as usize); let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((10) as usize); let __tmp_y = ('T' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((13) as usize); let __tmp_y = (':' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((16) as usize); let __tmp_y = (':' as i32) as u8; __tmp_x == __tmp_y }) {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), false);
    }
    { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((19) as usize, None)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };

        // Parse the fractional second.
    let mut nsec: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); let __tmp_y = 2; __tmp_x >= __tmp_y } && { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } && is_digit::<bytes>(s.clone(), Arc::new(Mutex::new(Some(1)))) {
        let mut n = Arc::new(Mutex::new(Some(2)));
        while { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); __tmp_x < __tmp_y } && is_digit::<bytes>(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let (__tmp_0, __tmp_1, __tmp_2) = parse_nanoseconds::<bytes>(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *nsec.lock().unwrap() = Some(__tmp_0); };
        { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize, None)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }

        // Parse the time zone.
    let mut t = date(Arc::new(Mutex::new(Some(year))), Arc::new(Mutex::new(Some(crate::r#mod::Month(Arc::new(Mutex::new(Some(month as i32))))))), Arc::new(Mutex::new(Some(day))), Arc::new(Mutex::new(Some(hour))), Arc::new(Mutex::new(Some(min))), Arc::new(Mutex::new(Some(sec))), Arc::new(Mutex::new(Some({ let __arg_holder = nsec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __arg_holder = UTC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); let __tmp_y = 1; __tmp_x != __tmp_y } || { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('Z' as i32) as u8; __tmp_x != __tmp_y } {
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).go_len() as i32); let __tmp_y = 6; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), false);
    }
        let mut hr = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((1) as usize, Some((3) as usize))))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(23)))) };
        let mut mm = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> = { let mut __f_guard = parseUint.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bytes>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> (i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).go_slice_to_string((4) as usize, Some((6) as usize))))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(59)))) };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } || !(({ let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y }) && { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((3) as usize); let __tmp_y = (':' as i32) as u8; __tmp_x == __tmp_y }) {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), false);
    }
        let mut zoneOffset = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = hr; let __tmp_y = 60; __tmp_x * __tmp_y }; let __tmp_y = mm; __tmp_x + __tmp_y }); let __tmp_y = 60; __tmp_x * __tmp_y })));
        if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).go_byte((0) as usize); let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } {
        { let __rhs = -1; let mut guard = zoneOffset.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }
        (*t.lock().unwrap().as_mut().unwrap()).add_sec(Arc::new(Mutex::new(Some(-((*zoneOffset.lock().unwrap().as_ref().unwrap()) as i64)))));
                // Use local zone with the given offset if possible.
        {
        let (_, mut offset, _, _, _) = { let __recv = local.clone(); let __recv_ptr: *mut crate::zoneinfo::Location = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::zoneinfo::Location }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some((*t.lock().unwrap().as_ref().unwrap()).unix_sec())))); __result };;
        if { let __tmp_x = offset; let __tmp_y = { let __v = (*zoneOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
            (*t.lock().unwrap().as_mut().unwrap()).set_loc(local.clone());;
        } else {
            (*t.lock().unwrap().as_mut().unwrap()).set_loc(fixed_zone(Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = zoneOffset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));;
        }
    }
    }
        // e.g., 07
        // e.g., 00
        // Use local zone with the given offset if possible.
    return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, true);
}

pub fn parse_strict_r_f_c3339(b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<crate::r#mod::Time>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut t, mut ok) = parse_r_f_c3339::<Vec<u8>>(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __arg_holder = Local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    if !ok {
        let (mut t, mut err) = parse(Arc::new(Mutex::new(Some(R_F_C3339.to_string()))), Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), err.clone());
    }
                // The parse template syntax cannot correctly validate RFC 3339.
                // Explicitly check for cases that Parse is unable to validate for.
                // See https://go.dev/issue/54580.
        let mut num2 = Arc::new(Mutex::new(Some(Box::new(move |b: Arc<Mutex<Option<Vec<u8>>>>| -> u8 {
        return {
            let __go_binary_0 = 10 as u8;
            let __go_binary_1 = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
            let __go_binary_2 = ('0' as i32) as u8;
            let __go_binary_3 = __go_binary_1 - __go_binary_2;
            let __go_binary_4 = __go_binary_0 * __go_binary_3;
            let __go_binary_5 = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() };
            let __go_binary_6 = ('0' as i32) as u8;
            let __go_binary_7 = __go_binary_5 - __go_binary_6;
            let __go_binary_8 = __go_binary_4 + __go_binary_7;
            __go_binary_8
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync>)));
        if true {
            return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
        } else if { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 11; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; let __tmp_y = (':' as i32) as u8; __tmp_x == __tmp_y } {
            return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new(ParseError { layout: Arc::new(Mutex::new(Some("2006-01-02T15:04:05Z07:00".to_string()))), value: Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), layout_elem: Arc::new(Mutex::new(Some("15".to_string()))), value_elem: Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ("2006-01-02T".len()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (1) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), message: Arc::new(Mutex::new(Some("".to_string()))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
        } else if { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[("2006-01-02T15:04:05".len()) as usize].clone() }; let __tmp_y = (',' as i32) as u8; __tmp_x == __tmp_y } {
            return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new(ParseError { layout: Arc::new(Mutex::new(Some("2006-01-02T15:04:05Z07:00".to_string()))), value: Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), layout_elem: Arc::new(Mutex::new(Some(".".to_string()))), value_elem: Arc::new(Mutex::new(Some(",".to_string()))), message: Arc::new(Mutex::new(Some("".to_string()))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
        } else if { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('Z' as i32) as u8; __tmp_x != __tmp_y } {
            if { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync> = { let mut __f_guard = num2.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 5; __tmp_x - __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))) }; let __tmp_y = 24 as u8; __tmp_x >= __tmp_y } {
            return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new(ParseError { layout: Arc::new(Mutex::new(Some("2006-01-02T15:04:05Z07:00".to_string()))), value: Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), layout_elem: Arc::new(Mutex::new(Some("Z07:00".to_string()))), value_elem: Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 6; __tmp_x - __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), message: Arc::new(Mutex::new(Some(": timezone hour out of range".to_string()))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
        } else if { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync> = { let mut __f_guard = num2.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<u8>>>>) -> u8 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x - __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))) }; let __tmp_y = 60 as u8; __tmp_x >= __tmp_y } {
            return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new(ParseError { layout: Arc::new(Mutex::new(Some("2006-01-02T15:04:05Z07:00".to_string()))), value: Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), layout_elem: Arc::new(Mutex::new(Some("Z07:00".to_string()))), value_elem: Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 6; __tmp_x - __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), message: Arc::new(Mutex::new(Some(": timezone minute out of range".to_string()))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
        }
        } else {
            return (Arc::new(Mutex::new(Some(Time { wall: Arc::new(Mutex::new(Some(0))), ext: Arc::new(Mutex::new(Some(0))), loc: Default::default() }))), Arc::new(Mutex::new(Some(Box::new(ParseError { layout: Arc::new(Mutex::new(Some("2006-01-02T15:04:05Z07:00".to_string()))), value: Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), layout_elem: Arc::new(Mutex::new(Some("2006-01-02T15:04:05Z07:00".to_string()))), value_elem: Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), message: Arc::new(Mutex::new(Some("".to_string()))), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
        }
    }
        // The parse template syntax cannot correctly validate RFC 3339.
        // Explicitly check for cases that Parse is unable to validate for.
        // See https://go.dev/issue/54580.
        // TODO(https://go.dev/issue/54580): Strict parsing is disabled for now.
        // Enable this again with a GODEBUG opt-out.
        // hour must be two digits
        // sub-second separator must be a period
        // timezone hour must be in range
        // timezone minute must be in range
        // unknown error; should not occur
    return ({ let __owned = t.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
}