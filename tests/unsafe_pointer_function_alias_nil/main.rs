use std::sync::{Arc, Mutex};

pub type Exporter = Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>>>>;


pub(crate) static exporter: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *exporter.lock().unwrap() = Some(Default::default());
}


pub fn r#use() -> Arc<Mutex<Option<String>>> {
    let mut exporterPtr = Arc::new(Mutex::new({ let __ptr = { let __target = exporter.clone(); let __guard = __target.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Exporter>(unimplemented!("unsafe.Pointer conversion to Exporter")) } }));
    if (*exporterPtr.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
    return { let __f_holder = ({ let __v = (*exporterPtr.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
}

fn main() {
    __go_init_all();
    println!("{}", format!("{}", (*r#use().lock().unwrap().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
