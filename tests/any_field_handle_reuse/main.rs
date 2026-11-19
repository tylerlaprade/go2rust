use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

#[derive(Clone, Default)]
pub struct entry {
    pub value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.value.clone();
        Self {
            value: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_any(self.value.lock().unwrap().as_ref().unwrap().as_ref()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn assign(e: Arc<Mutex<Option<entry>>>, value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
    { let new_val = value.clone(); (*e.lock().unwrap().as_mut().unwrap()).value = new_val; };
}

pub fn each(e: Arc<Mutex<Option<entry>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> () + Send + Sync>>>>) {
    {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> () + Send + Sync> = {
            let mut __f_guard = f.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            (*e.lock().unwrap().as_ref().unwrap()).value.clone()
        )
    };
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("ok".to_string()) as Box<dyn Any + Send + Sync>)));
    let mut e = Arc::new(Mutex::new(Some(entry { value: Default::default() })));
    assign(e.clone(), value.clone());
    each(
        e.clone(),
        Arc::new(Mutex::new(Some(Box::new(move |v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| {
        println!("{}", format!("{}", { let __text = format_any(v.lock().unwrap().as_ref().unwrap().as_ref()); __text }));
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> () + Send + Sync>)))
    );
}