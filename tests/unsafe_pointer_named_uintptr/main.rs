use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;


fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<raw>() { return "main.raw" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone, Default)]
pub struct raw(pub Arc<Mutex<Option<usize>>>);


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut zero: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    let mut p = Arc::new(Mutex::new(Some(raw(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some((*zero.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap()))))))));
    let mut value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new((*p.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>)));
    println!("{}", format!("{}", { let __tmp_x = (*Arc::new(Mutex::new(Some((*({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<raw>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.raw")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.raw")
        }
    }).0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y }));
}