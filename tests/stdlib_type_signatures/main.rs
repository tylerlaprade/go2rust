use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn label(file: Arc<Mutex<Option<go_ast::r#mod::File>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some("ok".to_string())))
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    println!("{}", format!("{}", (*label(Arc::new(Mutex::new(None))).lock().unwrap().as_ref().unwrap())));
}