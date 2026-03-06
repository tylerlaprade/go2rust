use std::sync::{Arc, Mutex};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut ctx, mut cancel) = (*context.lock().unwrap().as_mut().unwrap())::with_timeout(Arc::new(Mutex::new(Some((*context.lock().unwrap().as_mut().unwrap())::background()))), Arc::new(Mutex::new(Some(1 * (*(*time.lock().unwrap().as_mut().unwrap())::second.lock().unwrap().as_ref().unwrap())))));
    __defer_stack.push(Box::new(move || {
        (*cancel.lock().unwrap().as_ref().unwrap())();
    }));

    loop {
        if let Some(_) = (*time.lock().unwrap().as_mut().unwrap())::after(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap()))))).try_recv() {
            println!("{}", "Operation completed".to_string());
            break;
        }
        if let Some(_) = (*ctx.lock().unwrap().as_mut().unwrap()).done().try_recv() {
            println!("{} {}", "Context cancelled:".to_string(), format!("{}", (*((*ctx.lock().unwrap().as_mut().unwrap()).err()).lock().unwrap().as_ref().unwrap())));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}