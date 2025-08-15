use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn main() {
        // Slice with capacity
    let mut s = Arc::new(Mutex::new(Some(vec![0; 3])));
    print!("len={} cap={} {}\n", (*s.lock().unwrap().as_ref().unwrap()).len(), (*s.lock().unwrap().as_mut().unwrap()).capacity(), format_slice(&s));

        // Append beyond capacity
    {(*s.lock().unwrap().as_mut().unwrap()).extend(vec![1, 2, 3, 4, 5, 6, 7, 8]); s.clone()};
    print!("len={} cap={} {}\n", (*s.lock().unwrap().as_ref().unwrap()).len(), (*s.lock().unwrap().as_mut().unwrap()).capacity(), format_slice(&s));

        // Three-index slice
    let mut s2 = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap())[2 as usize..5 as usize].to_vec())));
    print!("s2: len={} cap={} {}\n", (*s2.lock().unwrap().as_ref().unwrap()).len(), (*s2.lock().unwrap().as_mut().unwrap()).capacity(), format_slice(&s2));

        // Copy
    let mut s3 = Arc::new(Mutex::new(Some(vec![0; 3])));
    let mut n = copy(s3.clone(), s.clone());
    print!("Copied {} elements: {}\n", (*n.lock().unwrap().as_mut().unwrap()), format_slice(&s3));

        // Nil slice vs empty slice
    let mut s4: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut s5 = Arc::new(Mutex::new(Some(vec![])));
    print!("s4==nil: {}, s5==nil: {}\n", (*s4.lock().unwrap()).is_none(), (*s5.lock().unwrap()).is_none());
}