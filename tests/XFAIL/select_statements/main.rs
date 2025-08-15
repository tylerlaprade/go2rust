use std::sync::{Arc, Mutex};

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
    let mut ch = ;

        // Non-blocking send
    // TODO: Unhandled statement type: SelectStmt

        // Non-blocking receive
    // TODO: Unhandled statement type: SelectStmt

        // Try to receive again (should hit default)
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
    let mut ch1 = ;
    let mut ch2 = ;

    // TODO: Unhandled statement type: SelectStmt

        // Read from both channels
    println!("{} {}", "Reading from ch1:".to_string(), <-(*ch1.lock().unwrap().as_mut().unwrap()));

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