use std::error::Error;
use std::sync::{Arc, Mutex};

pub fn divide(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*(*b.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) == 0 {
        return (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("division by zero".to_string())))));
    }
    return ({
            let __tmp_x = (*a.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_ref().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x / __tmp_y)))
        }, Arc::new(Mutex::new(None)));
}

fn main() {
    let (mut result, mut err) = divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }

    (result, err) = divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(0))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }
}