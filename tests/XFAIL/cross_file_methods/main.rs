mod methods;
mod types;
use methods::*;
use types::*;

use std::sync::{Arc, Mutex};

fn main() {
        // Test Counter methods - transpiler needs to know Counter has these methods
let mut c = Arc::new(Mutex::new(Some(Counter { value: Arc::new(Mutex::new(Some(10))) })));
    print!("Initial value: {}\n", (*(*c.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()));

    (*c.lock().unwrap().as_mut().unwrap()).increment();
    print!("After increment: {}\n", (*(*c.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()));

    (*c.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(5))));
    print!("After adding 5: {}\n", (*(*c.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()));

        // Test Point methods - transpiler needs to resolve method receivers
let mut p1 = Point { x: Arc::new(Mutex::new(Some(0))), y: Arc::new(Mutex::new(Some(0))) };
    let mut p2 = Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))) };

    let mut dist = (*p1.lock().unwrap().as_mut().unwrap()).distance(Arc::new(Mutex::new(Some((*p2.lock().unwrap().as_mut().unwrap())))));
    print!("Distance between points: {:.1}\n", (*dist.lock().unwrap().as_mut().unwrap()));

    (*p1.lock().unwrap().as_mut().unwrap()).move(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(1))));
    print!("After move: ({:.1}, {:.1})\n", (*(*p1.lock().unwrap().as_mut().unwrap()).x.lock().unwrap().as_ref().unwrap()), (*(*p1.lock().unwrap().as_mut().unwrap()).y.lock().unwrap().as_ref().unwrap()));

        // Test method on value vs pointer receiver
let mut newDist = (*p1.lock().unwrap().as_mut().unwrap()).distance(Arc::new(Mutex::new(Some((*p2.lock().unwrap().as_mut().unwrap())))));
    print!("New distance: {:.1}\n", (*newDist.lock().unwrap().as_mut().unwrap()));
}