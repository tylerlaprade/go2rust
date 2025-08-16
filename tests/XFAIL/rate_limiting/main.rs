use std::sync::{Arc, Mutex};

fn main() {
    let mut requests = ;
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (close.lock().unwrap().as_ref().unwrap())(requests.clone());

    let mut limiter = time.tick(Arc::new(Mutex::new(Some(100 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));

    for req in 0..(*requests.lock().unwrap().as_mut().unwrap()).len() {
        <-(*limiter.lock().unwrap().as_mut().unwrap());
        println!("{} {} {}", "request".to_string(), req, (*time.now().lock().unwrap().as_ref().unwrap()));
    }

    let mut burstyLimiter = ;

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    // TODO: Unhandled statement type: GoStmt

    let mut burstyRequests = ;
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (close.lock().unwrap().as_ref().unwrap())(burstyRequests.clone());
    for req in 0..(*burstyRequests.lock().unwrap().as_mut().unwrap()).len() {
        <-(*burstyLimiter.lock().unwrap().as_mut().unwrap());
        println!("{} {} {}", "request".to_string(), req, (*time.now().lock().unwrap().as_ref().unwrap()));
    }
}