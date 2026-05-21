use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn init_concurrency() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap();
}

fn main() {
    example_com_ifaceeq_event::__go_init_all();
    example_com_ifaceeq_keys::__go_init_all();
    example_com_ifaceeq_label::__go_init_all();

    __go_init_all();
    init_concurrency();
    println!("{}", format!("{}", "compiled".to_string()));
}

pub(crate) fn __go_init_all() {
}
