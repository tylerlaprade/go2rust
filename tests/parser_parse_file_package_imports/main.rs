use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
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
    strconv::__go_init_all();
    strings::__go_init_all();

    let mut fset = go_token::new_file_set();
    let (mut file, mut err) = go_parser::parse_file(fset.clone(), Arc::new(Mutex::new(Some("input.go".to_string()))), Arc::new(Mutex::new(Some(Box::new("package main\n\nimport (\n\t\"fmt\"\n\talias \"strings\"\n\t_ \"os\"\n)\n".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(go_parser::interface::Mode(Arc::new(Mutex::new(Some(go_parser::IMPORTS_ONLY as u64))))))));
    println!("{} {} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*(*(*file.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", ({ let __len_target = { let __field = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })));
    println!("{}", format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone()));
    println!("{} {}", format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone()));
    println!("{} {}", format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*{ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).imports.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone()));
}