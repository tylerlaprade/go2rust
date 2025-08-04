use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<HashMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
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

trait Shape {
    fn area(&self) -> Arc<Mutex<Option<f64>>>;
    fn perimeter(&self) -> Arc<Mutex<Option<f64>>>;
}

#[derive(Debug)]
struct Rectangle {
    width: Arc<Mutex<Option<f64>>>,
    height: Arc<Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Circle {
    radius: Arc<Mutex<Option<f64>>>,
}

impl Rectangle {
    pub fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }

    pub fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2.0 * ((*self.width.clone().lock().unwrap().as_mut().unwrap()) + (*self.height.clone().lock().unwrap().as_mut().unwrap())))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }
    fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2.0 * ((*self.width.clone().lock().unwrap().as_mut().unwrap()) + (*self.height.clone().lock().unwrap().as_mut().unwrap())))));
    }
}

impl Circle {
    pub fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }

    pub fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*2.0 * 3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
    fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*2.0 * 3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

pub fn print_shape_info(s: Arc<Mutex<Option<Box<dyn Shape>>>>) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", (*(*s.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_mut().unwrap()), (*(*s.lock().unwrap().as_mut().unwrap()).perimeter().lock().unwrap().as_mut().unwrap()));
}

fn main() {
    let mut rect = Rectangle { width: Arc::new(Mutex::new(Some(10))), height: Arc::new(Mutex::new(Some(5))) };
    let mut circle = Circle { radius: Arc::new(Mutex::new(Some(3))) };

    println!("{}", "Rectangle:".to_string());
    print_shape_info(rect.clone());

    println!("{}", "Circle:".to_string());
    print_shape_info(circle.clone());

    let mut shapes = Arc::new(Mutex::new(Some(vec![(*rect.lock().unwrap().as_mut().unwrap()), (*circle.lock().unwrap().as_mut().unwrap())])));
    println!("{}", "All shapes:".to_string());
    for (i, shape) in (*shapes.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Shape {}: ", i + 1);
        print_shape_info(Arc::new(Mutex::new(Some(shape))));
    }
}