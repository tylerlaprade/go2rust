use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i32>>>,
    pub y: Arc<Mutex<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, y: { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Point {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(Some(0))), y: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.y.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(10 as i32))), y: Arc::new(Mutex::new(Some(20 as i32))), ..Default::default() })));

        // Get pointer to X field
    let mut xPtr = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<i32>(unimplemented!("unsafe.Pointer conversion to i32")) } }));
    print!("X via unsafe: {}\n", { let __v = (*xPtr.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Get pointer to Y field
    let mut yPtr = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x + __tmp_y }))); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<i32>(unimplemented!("unsafe.Pointer conversion to i32")) } }));
    print!("Y via unsafe: {}\n", { let __v = (*yPtr.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Size and alignment
    print!("Size: {}, Align: {}\n", (*Arc::new(Mutex::new(Some(std::mem::size_of::<Point>()))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(std::mem::align_of::<Point>()))).lock().unwrap().as_ref().unwrap()));
}