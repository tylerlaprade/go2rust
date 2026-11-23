#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn init_concurrency() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
}

fn main() {
    ::example_com_ifaceeq_event::__go_init_all();
    ::example_com_ifaceeq_keys::__go_init_all();
    ::example_com_ifaceeq_label::__go_init_all();

    __go_init_all();
    init_concurrency();
    println!("{}", format!("{}", "compiled".to_string()));
}

pub(crate) fn __go_init_all() {
}
