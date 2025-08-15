use std::sync::{Arc, Mutex};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut ctx, mut cancel) = context.with_timeout(Arc::new(Mutex::new(Some(context.background()))), Arc::new(Mutex::new(Some(1 * (*(*time.lock().unwrap().as_mut().unwrap())::second.lock().unwrap().as_ref().unwrap())))));
    __defer_stack.push(Box::new(move || {
        (cancel.lock().unwrap().as_ref().unwrap())();
    }));

    // TODO: Unhandled statement type: SelectStmt

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}