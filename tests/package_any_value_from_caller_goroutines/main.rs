#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<callerError>() {
        format!("{}", v)
    } else if let Some(__go_text) = go_format_registered_any(value) {
        __go_text
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<f64>>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Point>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<callerError>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }

    if let Some(__go_value) = go_clone_registered_any(value) { return __go_value; }
    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_1_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct callerError {
    pub message: Arc<Mutex<Option<String>>>,
}

impl callerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.message.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            message: __go_clone_0_0,
        }
    }
}


impl Default for callerError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            message: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for callerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct pointerError {
    pub message: Arc<Mutex<Option<String>>>,
}

impl pointerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.message.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            message: __go_clone_0_0,
        }
    }
}


impl Default for pointerError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            message: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


impl callerError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.message.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl StdError for callerError {}


impl pointerError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.message.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl StdError for pointerError {}


/// A value put into an interface does not stay in the package that put it
/// there. main creates these values; another package prints and copies them.
fn main() {
    ::example_com_anyfromcaller_logger::__go_init_all();

    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some({ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("bad".to_string()))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } }))));
    example_com_anyfromcaller_logger::each(Arc::new(Mutex::new(Some(vec![{ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("later".to_string()))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } }]))));
    example_com_anyfromcaller_logger::each(Arc::new(Mutex::new(Some(vec![{ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("wrapped {}", 7)))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } }, Box::new("text".to_string()) as Box<dyn Any + Send + Sync>, Box::new(3i64) as Box<dyn Any + Send + Sync>]))));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some({ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("described".to_string()))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![1i64, 2i64]))).clone()) as Box<dyn Any + Send + Sync>))));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()]))).clone()) as Box<dyn Any + Send + Sync>))));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(1i64))))])))).clone()) as Box<dyn Any + Send + Sync>))));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Point>("main.Point", |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, None); Box::new(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Any + Send + Sync> }))));
    example_com_anyfromcaller_logger::each(Arc::new(Mutex::new(Some(vec![Box::new(Arc::new(Mutex::new(Some(vec![3i64, 4i64]))).clone()) as Box<dyn Any + Send + Sync>, Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("m".to_string(), Arc::new(Mutex::new(Some(2i64))))])))).clone()) as Box<dyn Any + Send + Sync>, Box::new(Arc::new(Mutex::new(Some(vec![1.5]))).clone()) as Box<dyn Any + Send + Sync>]))));
    println!("{} {}", format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec!["x".to_string()]))).clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))).clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut asError: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(callerError { message: Arc::new(Mutex::new(Some("typed error".to_string()))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some({ let __err_holder = asError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } }))));
    println!("{} {} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some({ let __err_holder = asError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some({ let __err_holder = asError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::classify(Arc::new(Mutex::new(Some({ let __err_holder = asError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())));
    let mut value = Arc::new(Mutex::new(Some(callerError { message: Arc::new(Mutex::new(Some("caller error".to_string()))), ..Default::default() })));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync> }))));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::classify(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())));
    example_com_anyfromcaller_logger::each(Arc::new(Mutex::new(Some(vec![{ go_register_any_runtime_type::<Point>("main.Point", |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, None); Box::new(Point { x: Arc::new(Mutex::new(Some(5))), y: Arc::new(Mutex::new(Some(6))), ..Default::default() }) as Box<dyn Any + Send + Sync> }, { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(callerError { message: Arc::new(Mutex::new(Some("copied error".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync> }]))));
    let mut boxed: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(callerError { message: Arc::new(Mutex::new(Some("preboxed".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync> })));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(boxed.clone()).lock().unwrap().as_ref().unwrap())));
    let mut pointerAsError: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(pointerError { message: Arc::new(Mutex::new(Some("typed pointer error".to_string()))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some({ let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } }))));
    println!("{} {} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some({ let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some({ let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::classify(Arc::new(Mutex::new(Some({ let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())));
    let mut pointer = Arc::new(Mutex::new(Some(pointerError { message: Arc::new(Mutex::new(Some("pointer error".to_string()))), ..Default::default() })));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(pointer.clone()) as Box<dyn Any + Send + Sync> }))));
    example_com_anyfromcaller_logger::each(Arc::new(Mutex::new(Some(vec![{ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(pointer.clone()) as Box<dyn Any + Send + Sync> }]))));
    println!("{} {} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(pointer.clone()) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(pointer.clone()) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::classify(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(pointer.clone()) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())));
    let mut numbers: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = example_com_anyfromcaller_logger::numbers().clone();
    println!("{}", format!("{}", { let __text = format_any(numbers.lock().unwrap().as_ref().unwrap().as_ref()); __text }));
    example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some(Box::new(7i64) as Box<dyn Any + Send + Sync>))));
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        example_com_anyfromcaller_logger::show(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Point>("main.Point", |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, None); Box::new(Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() }) as Box<dyn Any + Send + Sync> }))));;
        let mut asyncErr = Arc::new(Mutex::new(Some(callerError { message: Arc::new(Mutex::new(Some("async error".to_string()))), ..Default::default() })));;
        println!("{} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new({ let __arg_holder = asyncErr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new({ let __arg_holder = asyncErr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())));;
        example_com_anyfromcaller_logger::each(Arc::new(Mutex::new(Some(vec![{ go_register_any_runtime_type::<Point>("main.Point", |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, None); Box::new(Point { x: Arc::new(Mutex::new(Some(7))), y: Arc::new(Mutex::new(Some(8))), ..Default::default() }) as Box<dyn Any + Send + Sync> }, { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(callerError { message: Arc::new(Mutex::new(Some("async copied error".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync> }]))));;
        let mut asyncAsError: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(callerError { message: Arc::new(Mutex::new(Some("async typed error".to_string()))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));;
        println!("{}", format!("{}", (*example_com_anyfromcaller_logger::classify(Arc::new(Mutex::new(Some({ let __err_holder = asyncAsError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); std::sync::Arc::new(__typed.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); go_box_any_with_metadata(Arc::new(Mutex::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })))).lock().unwrap().as_ref().unwrap())));;
        let mut asyncPointer = Arc::new(Mutex::new(Some(pointerError { message: Arc::new(Mutex::new(Some("async pointer error".to_string()))), ..Default::default() })));;
        example_com_anyfromcaller_logger::each(Arc::new(Mutex::new(Some(vec![{ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(asyncPointer.clone()) as Box<dyn Any + Send + Sync> }]))));;
        println!("{} {} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(asyncPointer.clone()) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(asyncPointer.clone()) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::classify(Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Arc<Mutex<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.lock().unwrap()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any + Send + Sync> }, Some(|__value| { let __typed = __value.downcast_ref::<Arc<Mutex<Option<pointerError>>>>().expect("registered interface error type mismatch"); let __guard = __typed.lock().unwrap(); let __pointee = __guard.as_ref().expect("nil pointer cannot implement a non-nil error interface"); std::sync::Arc::new(__pointee.__go_value_clone()) as std::sync::Arc<dyn std::error::Error + Send + Sync> })); Box::new(asyncPointer.clone()) as Box<dyn Any + Send + Sync> })))).lock().unwrap().as_ref().unwrap())));;
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
}

impl GoValueClone for Point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for callerError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pointerError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
