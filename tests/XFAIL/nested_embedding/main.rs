use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct A {
    x: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct B {
    a: Arc<Mutex<Option<A>>>,
    y: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
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
    let mut c = C { b: Arc::new(Mutex::new(Some(B { a: Arc::new(Mutex::new(Some(A { x: Arc::new(Mutex::new(Some(10))) }))), y: Arc::new(Mutex::new(Some(20))) }))), z: Arc::new(Mutex::new(Some(30))) };

        // Direct access to nested promoted field
    print!("c.X = {}\n", (*c.b.lock().unwrap().as_ref().unwrap().a.lock().unwrap().as_ref().unwrap().x.lock().unwrap().as_ref().unwrap()));
    print!("c.Y = {}\n", (*c.b.lock().unwrap().as_ref().unwrap().y.lock().unwrap().as_ref().unwrap()));
    print!("c.Z = {}\n", (*c.z.lock().unwrap().as_ref().unwrap()));

        // Method accessing promoted field
    c.show_x();
}