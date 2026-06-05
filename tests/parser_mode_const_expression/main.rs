use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::sync::{Arc, Mutex};

fn main() {
    cmp::__go_init_all();
    go_ast::__go_init_all();
    go_parser::__go_init_all();
    go_scanner::__go_init_all();
    go_token::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    internal_filepathlite::__go_init_all();
    internal_stringslite::__go_init_all();
    path_filepath::__go_init_all();
    slices::__go_init_all();
    strings::__go_init_all();

    const mode: u64 = go_parser::ALL_ERRORS as u64 | go_parser::PARSE_COMMENTS as u64;

    { let (__tmp_0, __tmp_1) = go_parser::parse_file(go_token::new_file_set(), Arc::new(Mutex::new(Some("x.go".to_string()))), Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(("package main".to_string()).as_bytes().to_vec()))).clone()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(go_parser::interface::Mode(Arc::new(Mutex::new(Some(mode as u64)))))))); };
    println!("{}", format!("{}", "parsed".to_string()));
}