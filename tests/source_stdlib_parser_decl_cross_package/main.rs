use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    cmp::__go_init_all();
    example_com_source_stdlib_parser_decl_cross_package_walker::__go_init_all();
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

    let mut fset = go_token::new_file_set();
    let (mut file, mut err) = go_parser::parse_file(fset.clone(), Arc::new(Mutex::new(Some("input.go".to_string()))), Arc::new(Mutex::new(Some(Box::new("package main\nvar x int\n".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(go_parser::interface::Mode(Arc::new(Mutex::new(Some(0 as u64))))))));
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "parse failed".to_string()));
        return;
    }
    println!("{}", format!("{}", (*example_com_source_stdlib_parser_decl_cross_package_walker::decl_kind({ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone()).lock().unwrap().as_ref().unwrap())));
}