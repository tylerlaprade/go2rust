use go2rust_stdlib_stubs::*;
use std::thread;

pub fn init_concurrency() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap();
}

fn main() {
    __go_init_all();
    init_concurrency();
    println!("{}", format!("{}", "compiled".to_string()));
}

pub(crate) fn __go_init_all() {
}
