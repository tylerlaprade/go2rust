use go2rust_stdlib_stubs::*;

pub(crate) static uint64pow10: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 20]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *uint64pow10.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    {
        let mut __go_array = Vec::<u64>::with_capacity(20);
        __go_array.push(1 as u64);
        __go_array.push(1e1 as u64);
        __go_array.push(1e2 as u64);
        __go_array.push(1e3 as u64);
        __go_array.push(1e4 as u64);
        __go_array.push(1e5 as u64);
        __go_array.push(1e6 as u64);
        __go_array.push(1e7 as u64);
        __go_array.push(1e8 as u64);
        __go_array.push(1e9 as u64);
        __go_array.push(1e10 as u64);
        __go_array.push(1e11 as u64);
        __go_array.push(1e12 as u64);
        __go_array.push(1e13 as u64);
        __go_array.push(1e14 as u64);
        __go_array.push(1e15 as u64);
        __go_array.push(1e16 as u64);
        __go_array.push(1e17 as u64);
        __go_array.push(1e18 as u64);
        __go_array.push(1e19 as u64);
        let __go_array: [u64; 20] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *uint64pow10.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *uint64pow10.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_10() {
    {
        let mut __go_array = Vec::<u64>::with_capacity(20);
        __go_array.push(1 as u64);
        __go_array.push(1e1 as u64);
        __go_array.push(1e2 as u64);
        __go_array.push(1e3 as u64);
        __go_array.push(1e4 as u64);
        __go_array.push(1e5 as u64);
        __go_array.push(1e6 as u64);
        __go_array.push(1e7 as u64);
        __go_array.push(1e8 as u64);
        __go_array.push(1e9 as u64);
        __go_array.push(1e10 as u64);
        __go_array.push(1e11 as u64);
        __go_array.push(1e12 as u64);
        __go_array.push(1e13 as u64);
        __go_array.push(1e14 as u64);
        __go_array.push(1e15 as u64);
        __go_array.push(1e16 as u64);
        __go_array.push(1e17 as u64);
        __go_array.push(1e18 as u64);
        __go_array.push(1e19 as u64);
        let __go_array: [u64; 20] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *uint64pow10.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
