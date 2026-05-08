use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Default)]
pub struct counter {
    pub n: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}


impl counter {
    pub fn len(&self) -> Arc<Mutex<Option<i32>>> {
        return self.n.clone();
    }
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut c = Arc::new(Mutex::new(Some(counter { n: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    let mut xs = Arc::new(Mutex::new(Some(Vec::with_capacity((*(*c.lock().unwrap().as_mut().unwrap()).len().lock().unwrap().as_ref().unwrap()) as usize))));
    {(*xs.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![1, 2, 3]); xs.clone()};

    println!("{}", (*xs.lock().unwrap().as_ref().unwrap()).len());
    println!("{}", { let __v = (*xs.lock().unwrap().as_ref().unwrap()).clone(); __v }.capacity());
}