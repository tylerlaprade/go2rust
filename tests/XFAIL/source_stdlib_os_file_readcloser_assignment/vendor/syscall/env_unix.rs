use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub(crate) static envOnce: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::once::Once>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static envLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::rwmutex::RWMutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static env: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static envs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *envOnce.lock().unwrap() = Some(Default::default());
    *envLock.lock().unwrap() = Some(Default::default());
    *env.lock().unwrap() = Some(BTreeMap::new());
    *envs.lock().unwrap() = Some(vec![]);
    *envs.lock().unwrap() = Some((*runtime_envs().lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *envOnce.lock().unwrap() = Some(Default::default());
    *envLock.lock().unwrap() = Some(Default::default());
    *env.lock().unwrap() = Some(BTreeMap::new());
    *envs.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_0() {
    *envs.lock().unwrap() = Some((*runtime_envs().lock().unwrap().as_ref().unwrap()).clone());
}


pub fn runtime_envs() -> Arc<Mutex<Option<Vec<String>>>> {
    let __envs: Vec<String> = std::env::vars().map(|(__k, __v)| format!("{}={}", __k, __v)).collect();
    Arc::new(Mutex::new(Some(__envs)))
}


pub fn copyenv() {
    { let new_val = { let __collection_holder = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::new()))).clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *env.lock().unwrap() = new_val; };
    { let __range_holder = envs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, s) in __range_values.iter().enumerate() {
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (s.len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &(s); __s.as_bytes()[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('=' as i32) as u8; __tmp_x == __tmp_y } {
        let mut key = Arc::new(Mutex::new(Some({ let __s = &(s); let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() })));
        {
        let (_, mut ok) = { let __map = { let __map_holder = env.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*key.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if !ok {
            { let __map_key = (*key.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(i as i32))); (*env.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        } else {
            (*envs.lock().unwrap().as_mut().unwrap())[(i) as usize] = "".to_string();;
        }
    }
                // first mention of key
                // Clear duplicate keys. This permits Unsetenv to
                // safely delete only the first item without
                // worrying about unshadowing a later one,
                // which might be a security problem.
        break
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
}

pub fn getenv(key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut value: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        { let __once = (*envOnce.lock().unwrap().as_ref().unwrap()).clone(); __once.r#do(Arc::new(Mutex::new(Some(Box::new(move || { copyenv() }) as Box<dyn FnMut() -> () + Send + Sync>)))) };
        if { let __tmp_x = ((*key.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        {
        { let new_val = "".to_string(); *value.lock().unwrap() = Some(new_val); };;
        { let new_val = false; *found.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (value.clone(), (*found.lock().unwrap().as_ref().unwrap()));
    }
    }

        (*envLock.lock().unwrap().as_mut().unwrap()).r_lock();
        __defer_stack.push(Box::new(move || {
        (*envLock.lock().unwrap().as_mut().unwrap()).r_unlock();
    }));

        let (mut i, mut ok) = { let __map = { let __map_holder = env.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*key.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
        if !ok {
        {
        { let new_val = "".to_string(); *value.lock().unwrap() = Some(new_val); };;
        { let new_val = false; *found.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (value.clone(), (*found.lock().unwrap().as_ref().unwrap()));
    }
    }
        let mut s = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = envs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('=' as i32) as u8; __tmp_x == __tmp_y } {
        {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };;
        { let new_val = true; *found.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (value.clone(), (*found.lock().unwrap().as_ref().unwrap()));
    }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        {
        { let new_val = "".to_string(); *value.lock().unwrap() = Some(new_val); };;
        { let new_val = false; *found.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (value.clone(), (*found.lock().unwrap().as_ref().unwrap()));
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            (value.clone(), (*found.lock().unwrap().as_ref().unwrap()))
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
