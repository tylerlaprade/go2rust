#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


fn go_channel_after(duration: std::time::Duration) -> GoChannel<GoTime> {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        thread_channel.send(GoTime::now());
        thread_channel.close();
    });
    channel
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut words = GoChannel::<String>::new();
    let mut done = GoChannel::<AnonymousStruct1>::new();
    let done_thread = done.clone(); let words_thread = words.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        { let __range_holder = strings::fields(Arc::new(Mutex::new(Some("select works with strings".to_string())))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        words_thread.send((*w).clone());
    } };
        done_thread.close();;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let mut got: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    loop {
        {
    let __go_select_channel_0 = words.clone();
    let __go_select_channel_1 = done.clone();
    let __go_select_channel_2 = go_channel_after(std::time::Duration::from_secs(5)).clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let w = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut w = Arc::new(Mutex::new(Some(w)));
            { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = got.clone(); let __append_value = (*strings::to_upper(Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *got.lock().unwrap() = __slice_header; };
            continue;
            break;
        }
        if let Some(_) = __go_select_channel_1.try_recv_select() {
            break;
        }
        if let Some(_) = __go_select_channel_2.try_recv_select() {
            println!("{}", format!("{}", "timeout".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
        break
    }
    println!("{}", format!("{}", (*strings::join(got.clone(), Arc::new(Mutex::new(Some(" ".to_string())))).lock().unwrap().as_ref().unwrap())));
    let mut ticks = Arc::new(Mutex::new(Some(0i64)));
    let mut timer = go_channel_after(std::time::Duration::from_millis(20));
    let mut stop = Arc::new(Mutex::new(Some(false)));
    while !{ let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
    let __go_select_channel_0 = timer.clone();
    loop {
        if let Some(_) = __go_select_channel_0.try_recv_select() {
            { let new_val = true; *stop.lock().unwrap() = Some(new_val); };
            break;
        }
        { let mut guard = ticks.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        std::thread::sleep(std::time::Duration::from_nanos(1000000));
        break;
    }
    }
    }
    println!("{}", format!("{}", { let __tmp_x = { let __v = (*ticks.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y }));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}
