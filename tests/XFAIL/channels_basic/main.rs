use std::sync::{Arc, Mutex};

pub fn sender(ch: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        print!("Sending: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
        // TODO: Unhandled statement type: SendStmt
        time.sleep(Arc::new(Mutex::new(Some(100 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (close.lock().unwrap().as_ref().unwrap())(ch.clone());
}

pub fn receiver(ch: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    while true {
        let (mut value, mut ok) = <-(*ch.lock().unwrap().as_mut().unwrap());
        if !(*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Channel closed".to_string());
        break
    }
        print!("Received: {}\n", (*value.lock().unwrap().as_mut().unwrap()));
    }
}

fn main() {
        // Unbuffered channel
    let mut ch = ;

    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt

    time.sleep(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));

        // Buffered channel
    let mut buffered = ;
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt

    println!("{}", "Buffered channel contents:".to_string());
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        let mut msg = Arc::new(Mutex::new(Some(<-(*buffered.lock().unwrap().as_mut().unwrap()))));
        println!("{} {}", "Got:".to_string(), (*msg.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Channel range
    let mut numbers = ;
    // TODO: Unhandled statement type: GoStmt

    println!("{}", "Range over channel:".to_string());
    for num in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        println!("{} {}", "Number:".to_string(), num);
    }
}