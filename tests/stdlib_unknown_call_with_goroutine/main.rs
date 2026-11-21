use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    ::internal_filepathlite::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::path_filepath::__go_init_all();

    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send((*path_filepath::base(Arc::new(Mutex::new(Some("/tmp/go2rust".to_string())))).lock().unwrap().as_ref().unwrap()).clone());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
}