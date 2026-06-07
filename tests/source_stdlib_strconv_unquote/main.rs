use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    internal_stringslite::__go_init_all();
    strconv::__go_init_all();
    unicode_utf8::__go_init_all();

    let (mut value, mut err) = strconv::unquote(Arc::new(Mutex::new(Some("\"newline\\n\"".to_string()))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "error".to_string()));
        return;
    }
    print!("{}", format!("{}", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", "ok".to_string()));
}