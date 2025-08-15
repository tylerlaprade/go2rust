use std::any::Any;
use std::sync::{Arc, Mutex};

fn main() {
    let mut x: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(Some(Box::new("hello".to_string()) as Box<dyn Any>)));

        // Type assertion with comma-ok
let (mut s, mut ok) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "x is string:".to_string(), (*s.lock().unwrap().as_mut().unwrap()));
    }

        // Type assertion without comma-ok (would panic if wrong)
let mut str = Arc::new(Mutex::new(Some(({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }))));
    println!("{} {}", "Asserted string:".to_string(), (*str.lock().unwrap().as_mut().unwrap()));

        // Failed assertion with comma-ok
let (mut n, mut ok) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "x is int:".to_string(), (*n.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{}", "x is not an int".to_string());
    }
}