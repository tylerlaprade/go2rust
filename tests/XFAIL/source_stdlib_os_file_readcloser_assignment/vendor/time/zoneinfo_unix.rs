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

use crate::{zoneinfo::{Location, localLoc}, zoneinfo_read::{load_location_1}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static platformZoneSources: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *platformZoneSources.lock().unwrap() = Some(vec![]);
    {
        let mut __go_slice = Vec::<String>::with_capacity(4);
        __go_slice.push("/usr/share/zoneinfo/".to_string());
        __go_slice.push("/usr/share/lib/zoneinfo/".to_string());
        __go_slice.push("/usr/lib/locale/TZ/".to_string());
        __go_slice.push("/etc/zoneinfo".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *platformZoneSources.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_zero_globals() {
    *platformZoneSources.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_16() {
    {
        let mut __go_slice = Vec::<String>::with_capacity(4);
        __go_slice.push("/usr/share/zoneinfo/".to_string());
        __go_slice.push("/usr/share/lib/zoneinfo/".to_string());
        __go_slice.push("/usr/lib/locale/TZ/".to_string());
        __go_slice.push("/etc/zoneinfo".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *platformZoneSources.lock().unwrap() = Some(__go_slice);
    }
}


pub fn init_local() {
        // consult $TZ to find the time zone to use.
        // no $TZ means use the system default /etc/localtime.
        // $TZ="" means use UTC.
        // $TZ="foo" or $TZ=":foo" if foo is an absolute path, then the file pointed
        // by foo will be used to initialize timezone; otherwise, file
        // /usr/share/zoneinfo/foo will be used.
    let (mut tz, mut ok) = syscall::getenv(Arc::new(Mutex::new(Some("TZ".to_string()))));
    if !ok {
            let (mut z, mut err) = load_location_1(
                Arc::new(Mutex::new(Some("localtime".to_string()))),
                Arc::new(Mutex::new(Some(vec!["/etc".to_string()])))
            );
            if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; *localLoc.lock().unwrap() = Some(new_val); };
        { let new_val = "Local".to_string(); *(*localLoc.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
        return;
    }
        } else if { let __tmp_x = (*tz.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            if { let __tmp_x = { let __s = &((*tz.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (':' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*tz.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tz.lock().unwrap() = __moved_val; };
    }
            if { let __tmp_x = (*tz.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = { let __s = &((*tz.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('/' as i32) as u8; __tmp_x == __tmp_y } {
        {
        let (mut z, mut err) = load_location_1(
            Arc::new(Mutex::new(Some({ let __arg_holder = tz.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(vec!["".to_string()])))
        );;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            { let new_val = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; *localLoc.lock().unwrap() = Some(new_val); };;
            if { let __tmp_x = (*tz.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "/etc/localtime".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "Local".to_string(); *(*localLoc.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = tz.lock().unwrap().as_ref().unwrap().clone(); *(*localLoc.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    };
            return;;
        }
    }
    } else if { let __tmp_x = (*tz.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = (*tz.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "UTC".to_string(); __tmp_x != __tmp_y } {
        {
        let (mut z, mut err) = load_location_1(Arc::new(Mutex::new(Some({ let __arg_holder = tz.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), platformZoneSources.clone());;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            { let new_val = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; *localLoc.lock().unwrap() = Some(new_val); };;
            return;;
        }
    }
    }
        }

        // Fall back to UTC.
    { let new_val = "UTC".to_string(); *(*localLoc.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
