use std::sync::{Arc, Mutex};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut resp, mut err) = http.get(Arc::new(Mutex::new(Some("https://httpbin.org/json".to_string()))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    __defer_stack.push(Box::new(move || {
        (*resp.body.lock().unwrap().as_mut().unwrap()).close();
    }));

    let (mut body, _) = io.read_all(Arc::new(Mutex::new(Some(resp.body))));
    println!("{} {}", "Response:".to_string(), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(String::from_utf8((*body.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap())[..100 as usize].to_vec()))));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}