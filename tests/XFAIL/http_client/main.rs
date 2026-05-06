use std::sync::{Arc, Mutex};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut payload = Arc::new(Mutex::new(Some("{\"slideshow\":{\"author\":\"Yours Truly\",\"slides\":[1,2,3]}}".to_string())));
    let mut raw = Arc::new(Mutex::new(Some(format!("{}{}", "HTTP/1.1 200 OK\r\nContent-Length: 57\r\n\r\n".to_string(), { let __v = (*payload.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    let (mut resp, mut err) = http::read_response(Arc::new(Mutex::new(Some(bufio::new_reader(Arc::new(Mutex::new(Some(strings::new_reader(Arc::new(Mutex::new(Some({ let __v = (*raw.lock().unwrap().as_ref().unwrap()).clone(); __v }))))))))))), Arc::new(Mutex::new(Some(None))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let Body_defer_captured = Body.clone(); let resp_defer_captured = resp.clone(); __defer_stack.push(Box::new(move || {
        (*(*resp.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).close();
    }));

    let (mut body, _) = io::read_all(Arc::new(Mutex::new(Some((*{ let __field = (*resp.lock().unwrap().as_ref().unwrap()).body.clone(); __field }.lock().unwrap().as_ref().unwrap())))));
    let mut text = Arc::new(Mutex::new(Some(String::from_utf8((*body.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    if { let __tmp_x = (*text.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 100; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = (*text.lock().unwrap().as_ref().unwrap()).clone(); __s[..100 as usize].to_string() }))); *text.lock().unwrap() = new_val.lock().unwrap().take(); };
    }
    println!("{} {}", "Response:".to_string(), { let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v });

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}