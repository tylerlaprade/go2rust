use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Default)]
pub struct node {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });
    let mut n = Arc::new(Mutex::new(Some(node { value: Arc::new(Mutex::new(Some(7))), ..Default::default() })));
    let mut items: Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>> = Arc::new(Mutex::new(None));
    let mut result: Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>> = Arc::new(Mutex::new(None));
    {(*items.lock().unwrap()).get_or_insert_with(Vec::new).push(n.clone()); items.clone()};
    let result_closure_clone = result.clone(); let mut visit = Arc::new(Mutex::new(Some(Box::new(move |xs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>>| {
        { let __range_guard = xs.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for p in __range_values.iter() {
        {(*result_closure_clone.lock().unwrap()).get_or_insert_with(Vec::new).push(p.clone()); result_closure_clone.clone()};
    } }
    }) as Box<dyn Fn(Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>>) -> () + Send + Sync>)));
    { let __f_guard = visit.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(items.clone()) };
    println!("{}", (*result.lock().unwrap().as_ref().unwrap()).len());
}