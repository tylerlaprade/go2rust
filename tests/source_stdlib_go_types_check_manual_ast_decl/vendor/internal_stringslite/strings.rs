use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn has_prefix(s: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*prefix.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = ((*prefix.lock().unwrap().as_ref().unwrap()).len()) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}

pub fn has_suffix(s: Arc<Mutex<Option<String>>>, suffix: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*suffix.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*suffix.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x - __tmp_y }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*suffix.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}

pub fn index_byte(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn index(s: Arc<Mutex<Option<String>>>, substr: Arc<Mutex<Option<String>>>) -> i32 {
    let mut n = Arc::new(Mutex::new(Some((*substr.lock().unwrap().as_ref().unwrap()).len() as i32)));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            return 0;
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
            return index_byte(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __s = &((*substr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }))));
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x == __tmp_y } {
            if { let __tmp_x = (*substr.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*s.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return 0;
    }
            return -(1);
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x > __tmp_y } {
            return -(1);
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*internal_bytealg::MaxLen.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } {
                        // Use brute force when s and substr both are small
            if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = internal_bytealg::MAX_BRUTE_FORCE; __tmp_x <= __tmp_y } {
        return internal_bytealg::index_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
            let mut c0 = Arc::new(Mutex::new(Some({ let __s = &((*substr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })));
            let mut c1 = Arc::new(Mutex::new(Some({ let __s = &((*substr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })));
            let mut i = Arc::new(Mutex::new(Some(0)));
            let mut t = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y })));
            let mut fails = Arc::new(Mutex::new(Some(0)));
            while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = { let __v = (*c0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // IndexByte is faster than bytealg.IndexString, so use it as long as
                // we're not getting lots of false positives.
        let mut o = index_byte(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = o; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return -(1);
    }
        { let __rhs = { let __tmp_x = o; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // IndexByte is faster than bytealg.IndexString, so use it as long as
                // we're not getting lots of false positives.
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*substr.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = fails.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

                // Switch to bytealg.IndexString when IndexByte produces too many false positives.
        if { let __tmp_x = { let __v = (*fails.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_bytealg::cutover(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x > __tmp_y } {
        let mut r = internal_bytealg::index_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = r; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        return { let __tmp_x = r; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
    }
        return -(1);
    }
    }
                        // IndexByte is faster than bytealg.IndexString, so use it as long as
                        // we're not getting lots of false positives.
                        // Switch to bytealg.IndexString when IndexByte produces too many false positives.
            return -(1);
        }
        // Use brute force when s and substr both are small
        // IndexByte is faster than bytealg.IndexString, so use it as long as
        // we're not getting lots of false positives.
        // Switch to bytealg.IndexString when IndexByte produces too many false positives.
    let mut c0 = Arc::new(Mutex::new(Some({ let __s = &((*substr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })));
    let mut c1 = Arc::new(Mutex::new(Some({ let __s = &((*substr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })));
    let mut i = Arc::new(Mutex::new(Some(0)));
    let mut t = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut fails = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = { let __v = (*c0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        let mut o = index_byte(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = o; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return -(1);
    }
        { let __rhs = { let __tmp_x = o; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*substr.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = fails.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*fails.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = 4; let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y }; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // See comment in ../bytes/bytes.go.
        let mut j = internal_bytealg::index_rabin_karp::<String>(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = j; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return -(1);
    }
        return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = j; __tmp_x + __tmp_y };
    }
    }
        // See comment in ../bytes/bytes.go.
    -(1)
}

pub fn cut(s: Arc<Mutex<Option<String>>>, sep: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>, bool) {
    let mut before: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut after: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let mut i = index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            return (Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = (i as i32); let __tmp_y = ((*sep.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))), true);;
        }
    }
    return ({ let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some("".to_string()))), false);
}

pub fn clone(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let mut b = Arc::new(Mutex::new(Some(vec![0; ((*s.lock().unwrap().as_ref().unwrap()).len()) as usize])));
    { let _src = (*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min((*b.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*b.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    return Arc::new(Mutex::new(Some({ let __bytes_holder = b.clone(); let __bytes_guard = __bytes_holder.lock().unwrap(); let __bytes = __bytes_guard.as_ref().unwrap(); let __start = (0) as usize; let __len = __bytes.len(); let __end = __start + __len; String::from_utf8(__bytes[__start..__end].to_vec()).unwrap() })));
}