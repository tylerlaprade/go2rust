use std::sync::{Arc, Mutex};

pub fn worker(id: Arc<Mutex<Option<i32>>>, wg: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (*wg.lock().unwrap().as_mut().unwrap()).done();
    }));
    print!("Worker {} starting\n", (*id.lock().unwrap().as_mut().unwrap()));
    (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    print!("Worker {} done\n", (*id.lock().unwrap().as_mut().unwrap()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    let mut wg: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>;
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 3 {
        (*wg.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1))));
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*wg.lock().unwrap().as_mut().unwrap()).wait();
    println!("{}", "All workers done".to_string());
}