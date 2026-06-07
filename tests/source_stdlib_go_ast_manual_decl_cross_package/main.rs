use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    example_com_source_stdlib_go_ast_manual_decl_cross_package_walker::__go_init_all();
    go_ast::__go_init_all();
    go_token::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut file = Arc::new(Mutex::new(Some(go_ast::r#mod::File { name: go_ast::new_ident(Arc::new(Mutex::new(Some("main".to_string())))).clone(), decls: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GenDeclPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::GenDecl { tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)))]))), ..Default::default() })));
    println!("{}", format!("{}", (*example_com_source_stdlib_go_ast_manual_decl_cross_package_walker::decl_kinds({ let __field = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); __field }).lock().unwrap().as_ref().unwrap())));
}