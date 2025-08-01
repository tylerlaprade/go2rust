#[derive(Debug)]
struct List {
    head: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>,
    tail: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>,
}

#[derive(Debug)]
struct element {
    next: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>,
    val: std::sync::Arc<std::sync::Mutex<Option<T>>>,
}

impl Unknown {
    pub fn push(&mut self, v: std::sync::Arc<std::sync::Mutex<Option<T>>>) {
        if (*self.tail.lock().unwrap()).is_none() {
        { let new_val = (*.lock().unwrap()).clone(); *self.head.lock().unwrap() = new_val; };
        { let new_val = self.head.clone(); *self.tail.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = (*.lock().unwrap()).clone(); *self.tail.clone()::next.lock().unwrap() = new_val; };
        { let new_val = self.tail.clone()::next; *self.tail.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn map_keys(m: std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, V>>>>) -> std::sync::Arc<std::sync::Mutex<Option<Vec<K>>>> {

    let mut r = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 0])));
    for (k, _) in (*(*m.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).clone() {
        { let new_val = {(*r.lock().unwrap().as_mut().unwrap()).push(k); (*r.lock().unwrap().as_mut().unwrap())}; *r.lock().unwrap() = Some(new_val); };
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*r.lock().unwrap().as_mut().unwrap()).clone())));
}

fn main() {
    let mut m = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<i32, std::sync::Arc<std::sync::Mutex<Option<String>>>>::from([(1, std::sync::Arc::new(std::sync::Mutex::new(Some("2".to_string())))), (2, std::sync::Arc::new(std::sync::Mutex::new(Some("4".to_string())))), (4, std::sync::Arc::new(std::sync::Mutex::new(Some("8".to_string()))))]))));
    println!("{} {}", "keys:".to_string(), (*map_keys(m.clone()).lock().unwrap().as_mut().unwrap()));
    let mut lst = ;
    (*lst.lock().unwrap().as_mut().unwrap()).push(std::sync::Arc::new(std::sync::Mutex::new(Some(10))));
    (*lst.lock().unwrap().as_mut().unwrap()).push(std::sync::Arc::new(std::sync::Mutex::new(Some(13))));
    (*lst.lock().unwrap().as_mut().unwrap()).push(std::sync::Arc::new(std::sync::Mutex::new(Some(23))));
}