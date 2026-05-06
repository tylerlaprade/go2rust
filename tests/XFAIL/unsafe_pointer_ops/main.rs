use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct Point {
    pub x: Arc<Mutex<Option<i32>>>,
    pub y: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.y.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(10))), y: Arc::new(Mutex::new(Some(20))), ..Default::default() })));

        // Get pointer to X field
    let mut xPtr = { let __f_holder = ({ let __v = (*int32.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __f_guard = __f_holder.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)((*p.lock().unwrap().as_ref().unwrap())) };
    print!("X via unsafe: {}\n", { let __v = (*xPtr.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Get pointer to Y field
    let mut yPtr = { let __f_holder = ({ let __v = (*int32.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __f_guard = __f_holder.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)({ let __tmp_x = (*Arc::new(Mutex::new(Some((*(*p.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*unsafe::offsetof(Arc::new(Mutex::new(Some((*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) };
    print!("Y via unsafe: {}\n", { let __v = (*yPtr.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Size and alignment
    print!("Size: {}, Align: {}\n", (*unsafe::sizeof(Arc::new(Mutex::new(Some({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()), (*unsafe::alignof(Arc::new(Mutex::new(Some({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()));
}