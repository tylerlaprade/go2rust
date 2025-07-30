pub fn basic_select() {
    let mut ch1 = ;
    let mut ch2 = ;
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: SelectStmt
}

pub fn select_with_timeout() {
    let mut ch = ;
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: SelectStmt
}

pub fn select_with_default() {
    let mut ch = vec![0; 1];
    // TODO: Unhandled statement type: SelectStmt
    // TODO: Unhandled statement type: SelectStmt
    // TODO: Unhandled statement type: SelectStmt
}

pub fn select_loop() {
    let mut ch1 = ;
    let mut ch2 = ;
    let mut quit = ;
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    println!("{}", "Starting select loop:".to_string());
    while true {
        // TODO: Unhandled statement type: SelectStmt
    }
}

pub fn select_with_send() {
    let mut ch1 = vec![0; 1];
    let mut ch2 = vec![0; 1];
    // TODO: Unhandled statement type: SelectStmt
    println!("{} {}", "Reading from ch1:".to_string(), <-(*ch1.lock().unwrap().as_ref().unwrap()));
    // TODO: Unhandled statement type: SelectStmt
}

fn main() {
    println!("{}", "=== Basic select ===".to_string());
    basic_select();
    println!("{}", "\n=== Select with timeout ===".to_string());
    select_with_timeout();
    println!("{}", "\n=== Select with default ===".to_string());
    select_with_default();
    println!("{}", "\n=== Select with send ===".to_string());
    select_with_send();
    println!("{}", "\n=== Select loop ===".to_string());
    select_loop();
    println!("{}", "\n=== All examples completed ===".to_string());
}