use std::any::Any;
use std::sync::{Arc, Mutex};

fn main() {
    let mut x: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(Some(Box::new("hello".to_string()) as Box<dyn Any>)));
    let (mut s, mut ok) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    });
    println!("{} {}", (*s.lock().unwrap().as_mut().unwrap()), (*ok.lock().unwrap().as_mut().unwrap()));
}