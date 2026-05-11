use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Default)]
pub struct queue {
    pub later: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>>>>,
}

impl queue {
    pub fn __go_value_clone(&self) -> Self {
        Self { later: self.later.clone() }
    }
}

impl std::fmt::Display for queue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.later.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", std::iter::repeat("<func>").take(__v.len()).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } })
    }
}


impl queue {
    pub fn add(&mut self, r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
        { let new_val = { let __append_target = self.later.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(r#fn.clone()); __append_target.clone() }; self.later = new_val; };
    }
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut q = Arc::new(Mutex::new(Some(queue { later: Arc::new(Mutex::new(Some(Vec::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>::new()))), ..Default::default() })));
    (*q.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{}", "later".to_string());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    let mut f = { let __seq = { let __seq_holder = (*q.lock().unwrap().as_ref().unwrap()).later.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
    { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
}