use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct A {
    x: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct B {
    a: Arc<Mutex<Option<A>>>,
    y: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct C {
    b: Arc<Mutex<Option<B>>>,
    z: Arc<Mutex<Option<i32>>>,
}

impl C {
    pub fn show_x(&self) {
        print!("X = {}\n", (*self.x.lock().unwrap().as_ref().unwrap()));
    }
}

impl B {
}

fn main() {
    let mut c = Arc::new(Mutex::new(Some(C { b: Arc::new(Mutex::new(Some(B { a: Arc::new(Mutex::new(Some(A { x: Arc::new(Mutex::new(Some(10))) }))), y: Arc::new(Mutex::new(Some(20))) }))), z: Arc::new(Mutex::new(Some(30))) })));

        // Direct access to nested promoted field
    print!("c.X = {}\n", (*(*(*c.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap().a.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()));
    print!("c.Y = {}\n", (*(*(*c.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()));
    print!("c.Z = {}\n", (*(*c.lock().unwrap().as_ref().unwrap()).z.lock().unwrap().as_ref().unwrap()));

        // Method accessing promoted field
    (*c.lock().unwrap().as_mut().unwrap()).show_x();
}