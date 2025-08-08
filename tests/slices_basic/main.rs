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
    let mut slice = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {}", "Original slice:".to_string(), format_slice(&slice));

    {(*slice.lock().unwrap().as_mut().unwrap()).extend(vec![6, 7]); slice.clone()};
    println!("{} {}", "After append:".to_string(), format_slice(&slice));

    let mut subSlice = Arc::new(Mutex::new(Some((*slice.lock().unwrap().as_ref().unwrap())[1 as usize..4 as usize].to_vec())));
    println!("{} {}", "Sub-slice [1:4]:".to_string(), format_slice(&subSlice));

    println!("{} {}", "Length:".to_string(), (*slice.lock().unwrap().as_ref().unwrap()).len());
    println!("{} {}", "Capacity:".to_string(), (*slice.lock().unwrap().as_mut().unwrap()).capacity());

    let mut made = Arc::new(Mutex::new(Some(vec![0; 3])));
    (*made.lock().unwrap().as_mut().unwrap())[0] = 10;
    (*made.lock().unwrap().as_mut().unwrap())[1] = 20;
    (*made.lock().unwrap().as_mut().unwrap())[2] = 30;
    println!("{} {}", "Made slice:".to_string(), format_slice(&made));
}