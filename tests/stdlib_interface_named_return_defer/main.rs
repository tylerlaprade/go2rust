use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic;

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Basic>")
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Pointer;

impl std::fmt::Display for types_Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Pointer>")
    }
}


impl types_Pointer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Type;

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}


impl types_Type {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


impl From<types_Pointer> for types_Type {
    fn from(_value: types_Pointer) -> Self {
        Self::default()
    }
}


pub mod types {
    use super::*;
    pub fn Typ() -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<types_Basic>>>>>>> {
        Arc::new(Mutex::new(Some::<Vec<Arc<Mutex<Option<types_Basic>>>>>(Default::default())))
    }

    pub fn new_pointer<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Pointer>>> {
        Arc::new(Mutex::new(Some::<types_Pointer>(Default::default())))
    }
}


pub fn make_type() -> Arc<Mutex<Option<types_Type>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Arc<Mutex<Option<types_Type>>> = Arc::new(Mutex::new(Some(Default::default())));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    {
        { let new_val = { let __arg = types::new_pointer({ let __seq = { let __seq_holder = types::Typ().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[2usize].clone() }); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res
    }
}

fn main() {
    if false {
        println!("{}", (*make_type().lock().unwrap()).is_some());
    }
    println!("{}", "ok".to_string());
}