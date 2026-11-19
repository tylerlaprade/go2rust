use go2rust_stdlib_stubs::*;

pub(crate) const FN_PARSE_FLOAT: &'static str = "ParseFloat";


pub(crate) static optimize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static powtab: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static float64pow10: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<f64>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static float32pow10: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<f32>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *optimize.lock().unwrap() = Some(false);
    *powtab.lock().unwrap() = Some(vec![]);
    *float64pow10.lock().unwrap() = Some(vec![]);
    *float32pow10.lock().unwrap() = Some(vec![]);
    *optimize.lock().unwrap() = Some(true);
    {
        let mut __go_slice = Vec::<i32>::with_capacity(9);
        __go_slice.push(1);
        __go_slice.push(3);
        __go_slice.push(6);
        __go_slice.push(9);
        __go_slice.push(13);
        __go_slice.push(16);
        __go_slice.push(19);
        __go_slice.push(23);
        __go_slice.push(26);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *powtab.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<f64>::with_capacity(23);
        __go_slice.push(1e0);
        __go_slice.push(1e1);
        __go_slice.push(1e2);
        __go_slice.push(1e3);
        __go_slice.push(1e4);
        __go_slice.push(1e5);
        __go_slice.push(1e6);
        __go_slice.push(1e7);
        __go_slice.push(1e8);
        __go_slice.push(1e9);
        __go_slice.push(1e10);
        __go_slice.push(1e11);
        __go_slice.push(1e12);
        __go_slice.push(1e13);
        __go_slice.push(1e14);
        __go_slice.push(1e15);
        __go_slice.push(1e16);
        __go_slice.push(1e17);
        __go_slice.push(1e18);
        __go_slice.push(1e19);
        __go_slice.push(1e20);
        __go_slice.push(1e21);
        __go_slice.push(1e22);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *float64pow10.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<f32>::with_capacity(11);
        __go_slice.push(1e0);
        __go_slice.push(1e1);
        __go_slice.push(1e2);
        __go_slice.push(1e3);
        __go_slice.push(1e4);
        __go_slice.push(1e5);
        __go_slice.push(1e6);
        __go_slice.push(1e7);
        __go_slice.push(1e8);
        __go_slice.push(1e9);
        __go_slice.push(1e10);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *float32pow10.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_zero_globals() {
    *optimize.lock().unwrap() = Some(false);
    *powtab.lock().unwrap() = Some(vec![]);
    *float64pow10.lock().unwrap() = Some(vec![]);
    *float32pow10.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_0() {
    *optimize.lock().unwrap() = Some(true);
}


pub(crate) fn __go_init_order_1() {
    {
        let mut __go_slice = Vec::<i32>::with_capacity(9);
        __go_slice.push(1);
        __go_slice.push(3);
        __go_slice.push(6);
        __go_slice.push(9);
        __go_slice.push(13);
        __go_slice.push(16);
        __go_slice.push(19);
        __go_slice.push(23);
        __go_slice.push(26);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *powtab.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_2() {
    {
        let mut __go_slice = Vec::<f64>::with_capacity(23);
        __go_slice.push(1e0);
        __go_slice.push(1e1);
        __go_slice.push(1e2);
        __go_slice.push(1e3);
        __go_slice.push(1e4);
        __go_slice.push(1e5);
        __go_slice.push(1e6);
        __go_slice.push(1e7);
        __go_slice.push(1e8);
        __go_slice.push(1e9);
        __go_slice.push(1e10);
        __go_slice.push(1e11);
        __go_slice.push(1e12);
        __go_slice.push(1e13);
        __go_slice.push(1e14);
        __go_slice.push(1e15);
        __go_slice.push(1e16);
        __go_slice.push(1e17);
        __go_slice.push(1e18);
        __go_slice.push(1e19);
        __go_slice.push(1e20);
        __go_slice.push(1e21);
        __go_slice.push(1e22);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *float64pow10.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_3() {
    {
        let mut __go_slice = Vec::<f32>::with_capacity(11);
        __go_slice.push(1e0);
        __go_slice.push(1e1);
        __go_slice.push(1e2);
        __go_slice.push(1e3);
        __go_slice.push(1e4);
        __go_slice.push(1e5);
        __go_slice.push(1e6);
        __go_slice.push(1e7);
        __go_slice.push(1e8);
        __go_slice.push(1e9);
        __go_slice.push(1e10);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *float32pow10.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
