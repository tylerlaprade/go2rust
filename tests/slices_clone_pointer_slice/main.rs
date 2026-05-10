use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct pkg {
    pub path: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for pkg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.path.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut pkgs = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(pkg { path: Arc::new(Mutex::new(Some("root".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(pkg { path: Arc::new(Mutex::new(Some("dep".to_string()))), ..Default::default() })))])));
    let mut list = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = pkgs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone())));
    println!("{}", (*list.lock().unwrap().as_ref().unwrap()).len());
}