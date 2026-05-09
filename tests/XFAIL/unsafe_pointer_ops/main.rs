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
    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(10 as i32))), y: Arc::new(Mutex::new(Some(20 as i32))), ..Default::default() })));

        // Get pointer to X field
    let mut xPtr = Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize)));
    print!("X via unsafe: {}\n", { let __v = (*xPtr.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Get pointer to Y field
    let mut yPtr = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x + __tmp_y })));
    print!("Y via unsafe: {}\n", { let __v = (*yPtr.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Size and alignment
    print!("Size: {}, Align: {}\n", (*Arc::new(Mutex::new(Some(std::mem::size_of::<Point>()))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(std::mem::align_of::<Point>()))).lock().unwrap().as_ref().unwrap()));
}