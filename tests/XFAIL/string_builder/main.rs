use std::sync::{Arc, Mutex};

fn main() {
    let mut builder: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>;
    builder.write_string(Arc::new(Mutex::new(Some("Hello".to_string()))));
    builder.write_string(Arc::new(Mutex::new(Some(" ".to_string()))));
    builder.write_string(Arc::new(Mutex::new(Some("World".to_string()))));
    let mut result = builder.string();
    println!("{}", (*result.lock().unwrap().as_mut().unwrap()));
}