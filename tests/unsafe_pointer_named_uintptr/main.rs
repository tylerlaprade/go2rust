use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Default)]
pub struct raw(pub Arc<Mutex<Option<usize>>>);


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut zero: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    let mut p = Arc::new(Mutex::new(Some(raw(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some((*zero.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap()))))))));
    let mut value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new((*p.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>)));
    println!("{}", { let __tmp_x = (*Arc::new(Mutex::new(Some((*({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<raw>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }).0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y });
}