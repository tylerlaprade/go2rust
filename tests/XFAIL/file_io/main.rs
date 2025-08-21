use std::sync::{Arc, Mutex};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut file, mut err) = (*os.lock().unwrap().as_mut().unwrap()).create(Arc::new(Mutex::new(Some("test.txt".to_string()))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    __defer_stack.push(Box::new(move || {
        (*file.lock().unwrap().as_mut().unwrap()).close();
    }));

    (*file.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("Hello, World!".to_string()))));
    println!("{}", "File written successfully".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}