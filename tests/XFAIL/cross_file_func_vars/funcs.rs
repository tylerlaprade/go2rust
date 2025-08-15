use std::sync::{Arc, Mutex};

/// Regular function for comparison
pub fn regular_double(x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = 2;
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
}

/// Function that returns a function
pub fn make_multiplier(factor: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> {

    return Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*factor.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn init() {
        // Assign function to variable in init
{ let new_val = Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("Dynamic: {}", (*s.lock().unwrap().as_mut().unwrap())))));
    }) as Box<dyn Fn(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>))); *DYNAMIC_FUNC.lock().unwrap() = Some(new_val); };
}