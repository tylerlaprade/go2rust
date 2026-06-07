use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_abi::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_sync::__go_init_all();
    ::io::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();

    io::multi_writer(Arc::new(Mutex::new(Some(vec![{ let __field = io::Discard.clone(); __field }, { let __field = io::Discard.clone(); __field }]))));
    println!("{}", format!("{}", "ok".to_string()));
}