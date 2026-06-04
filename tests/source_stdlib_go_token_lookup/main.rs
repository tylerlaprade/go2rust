use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    go_token::__go_init_all();

    let mut keyword = go_token::lookup(Arc::new(Mutex::new(Some("func".to_string()))));
    let mut ident = go_token::lookup(Arc::new(Mutex::new(Some("not_keyword".to_string()))));
    println!("{} {}", format!("{}", (*go_token::r#mod::Token::string(&(*keyword.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())), format!("{}", go_token::r#mod::Token::is_keyword(&(*keyword.lock().unwrap().as_ref().unwrap()))));
    println!("{} {}", format!("{}", (*go_token::r#mod::Token::string(&(*ident.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())), format!("{}", go_token::r#mod::Token::is_keyword(&(*ident.lock().unwrap().as_ref().unwrap()))));
    println!("{} {}", format!("{}", go_token::position::Pos::is_valid(&(go_token::position::Pos(Arc::new(Mutex::new(Some(1 as i32))))))), format!("{}", go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))).is_valid()));
}