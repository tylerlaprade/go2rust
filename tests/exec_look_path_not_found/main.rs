use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub mod exec {
    use super::*;
    pub fn look_path<T0>(_arg0: T0) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<String>(Default::default()))), Arc::new(Mutex::new(Some::<Box<dyn StdError + Send + Sync>>(Box::<dyn StdError + Send + Sync>::from("executable file not found")))))
    }
}


fn main() {
    let (_, mut err) = exec::look_path("__go2rust_missing_executable__".to_string());
    println!("{}", format!("{}", (*err.lock().unwrap()).is_some()));
}