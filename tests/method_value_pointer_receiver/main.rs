use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct tracker {
    pub value: Arc<Mutex<Option<i64>>>,
}

impl tracker {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for tracker {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl tracker {
    pub fn bump(&mut self) {
        { let __target = self.value.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn make_bump(t: Arc<Mutex<Option<tracker>>>) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> {
    Arc::new(Mutex::new(Some({ let __recv = t.clone(); Box::new(move || { (*__recv.lock().unwrap().as_mut().unwrap()).bump() }) as Box<dyn FnMut() -> () + Send + Sync> })))
}

fn main() {
    let mut t = Arc::new(Mutex::new(Some(tracker { value: Arc::new(Mutex::new(Some(0))) })));
    let mut first = make_bump(t.clone());
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = first.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    let mut second: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let new_val = Arc::new(Mutex::new(Some({ let __recv = t.clone(); Box::new(move || { (*__recv.lock().unwrap().as_mut().unwrap()).bump() }) as Box<dyn FnMut() -> () + Send + Sync> }))); second = new_val; };
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = second.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    std::thread::spawn(move || {
        ;
    });
    println!("{}", format!("{}", (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap())));
}