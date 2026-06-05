use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    go_token::__go_init_all();

    let mut obj = types::new_type_name(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))), (), "T".to_string(), ());
    let _ = types::new_type_param(obj.clone(), ());
    println!("{}", format!("{}", (*obj.lock().unwrap()).is_some()));
}