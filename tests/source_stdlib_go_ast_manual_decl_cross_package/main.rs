#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::example_com_source_stdlib_go_ast_manual_decl_cross_package_walker::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_token::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut file = Arc::new(Mutex::new(Some(go_ast::r#mod::File {
        name: go_ast::new_ident(Arc::new(Mutex::new(Some("main".to_string())))).clone(),
        decls: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GenDeclPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::GenDecl { tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i64))))))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)))]))),
        ..Default::default()
    })));
    println!("{}", format!("{}", (*example_com_source_stdlib_go_ast_manual_decl_cross_package_walker::decl_kinds({ let __field = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); __field }).lock().unwrap().as_ref().unwrap())));
}