use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub env: Arc<Mutex<Option<Vec<String>>>>,
    pub dir: Arc<Mutex<Option<String>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.env.clone();
        let __go_clone_1_0 = { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            env: __go_clone_0_0,
            dir: __go_clone_1_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            env: __go_default_0_0,
            dir: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.env));
        let __go_fmt_1 = format!("{}", (*self.dir.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn build_env(cfg: Arc<Mutex<Option<Config>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    { let __append_target = { let __result = slices::clip::<Vec<String>, String>({ let __slice_holder = (*cfg.lock().unwrap().as_ref().unwrap()).env.clone(); { let __slice_guard = __slice_holder.lock().unwrap(); Arc::new(Mutex::new(__slice_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Arc::new(Mutex::new(Some(__elem)))).collect::<Vec<_>>()))) } }); let __result_guard = __result.lock().unwrap(); Arc::new(Mutex::new(__result_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem.lock().unwrap().as_ref().unwrap()).clone()).collect::<Vec<_>>()))) }.clone(); let __append_value = format!("{}{}", "PWD=".to_string(), (*{ let __field = (*cfg.lock().unwrap().as_ref().unwrap()).dir.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }
}

fn main() {
    ::cmp::__go_init_all();
    ::iter::__go_init_all();
    ::math_bits::__go_init_all();
    ::slices::__go_init_all();

    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
    let mut cfg = Arc::new(Mutex::new(Some(Config { env: Arc::new(Mutex::new(Some(vec!["A=1".to_string()]))), dir: Arc::new(Mutex::new(Some("/tmp/work".to_string()))), ..Default::default() })));
    let mut env = build_env(cfg.clone());
    println!("{} {} {}", format!("{}", (*env.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __seq = { let __seq_holder = env.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = env.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
}

impl GoValueClone for Config {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
