use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct node {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });
    let mut n = Arc::new(Mutex::new(Some(node { value: Arc::new(Mutex::new(Some(7))), ..Default::default() })));
    let mut items: Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>> = Arc::new(Mutex::new(None));
    let mut result: Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>> = Arc::new(Mutex::new(None));
    { let new_val = { let __append_target = items.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(n.clone()); __append_target.clone() }; items = new_val; };
    let mut result_closure_clone = result.clone(); let mut visit = Arc::new(Mutex::new(Some(Box::new(move |xs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>>| {
        { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        { let __append_target = result_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*p).clone()); __append_target.clone() };
    } }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>>) -> () + Send + Sync>)));
    {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>>) -> () + Send + Sync> = {
            let mut __f_guard = visit.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Vec<Arc<Mutex<Option<node>>>>>>>) -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            items.clone()
        )
    };
    println!("{}", format!("{}", (*result.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}