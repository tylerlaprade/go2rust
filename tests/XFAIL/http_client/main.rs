use std::sync::{Arc, Mutex};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut resp, mut err) = (*http.lock().unwrap().as_mut().unwrap()).get(Arc::new(Mutex::new(Some("https://httpbin.org/json".to_string()))));
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
        (*resp.lock().unwrap().as_mut().unwrap()).body.close();
    }));

    let (mut body, _) = (*io.lock().unwrap().as_mut().unwrap()).read_all(Arc::new(Mutex::new(Some((*resp.lock().unwrap().as_mut().unwrap()).body))));
    println!("{} {}", "Response:".to_string(), (string.lock().unwrap().as_ref().unwrap())(body.clone())[..100].to_vec());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}