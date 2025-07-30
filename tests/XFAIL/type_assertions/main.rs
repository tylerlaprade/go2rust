pub fn process_value(value: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>>) {
    let (mut (*str.lock().unwrap().as_ref().unwrap()), mut (*ok.lock().unwrap().as_ref().unwrap())) = match (*value.lock().unwrap().as_ref().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    if (*ok.lock().unwrap().as_ref().unwrap()) {
        print!("String value: {} (length: {})\n", (*str.lock().unwrap().as_ref().unwrap()), (*str.lock().unwrap().as_ref().unwrap()).len());
        return;
    }
    let (mut (*num.lock().unwrap().as_ref().unwrap()), mut (*ok.lock().unwrap().as_ref().unwrap())) = match (*value.lock().unwrap().as_ref().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_ref().unwrap()) {
        print!("Integer value: {} (doubled: {})\n", (*num.lock().unwrap().as_ref().unwrap()), (*num.lock().unwrap().as_ref().unwrap()) * 2);
        return;
    }
    let (mut (*f.lock().unwrap().as_ref().unwrap()), mut (*ok.lock().unwrap().as_ref().unwrap())) = match (*value.lock().unwrap().as_ref().unwrap()).downcast_ref::<f64>() { Some(v) => (v.clone(), true), None => (0.0, false) };
    if (*ok.lock().unwrap().as_ref().unwrap()) {
        print!("Float value: {:.2} (squared: {:.2})\n", (*f.lock().unwrap().as_ref().unwrap()), (*f.lock().unwrap().as_ref().unwrap()) * (*f.lock().unwrap().as_ref().unwrap()));
        return;
    }
    print!("Unknown type: %T with value: {}\n", (*value.lock().unwrap().as_ref().unwrap()), (*value.lock().unwrap().as_ref().unwrap()));
}

pub fn assert_without_check(value: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>>) {
    
    let mut str = std::sync::Arc::new(std::sync::Mutex::new(Some(match (*value.lock().unwrap().as_ref().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) })));
    print!("Asserted string: {}\n", (*str.lock().unwrap().as_ref().unwrap()));
}



#[derive(Debug)]
struct Rectangle {
    width: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
    height: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

pub fn area() -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()).width * (*r.lock().unwrap().as_ref().unwrap()).height)));
}

#[derive(Debug)]
struct Circle {
    radius: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

pub fn area() -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(3.14159 * (*c.lock().unwrap().as_ref().unwrap()).radius * (*c.lock().unwrap().as_ref().unwrap()).radius)));
}

pub fn describe_shape(s: std::sync::Arc<std::sync::Mutex<Option<Shape>>>) {
    print!("Shape area: {:.2}\n", (*s.lock().unwrap().as_ref().unwrap()).area());
    let (mut (*rect.lock().unwrap().as_ref().unwrap()), mut (*ok.lock().unwrap().as_ref().unwrap())) = match (*s.lock().unwrap().as_ref().unwrap()).downcast_ref::<Rectangle>() { Some(v) => (v.clone(), true), None => (Default::default(), false) };
    if (*ok.lock().unwrap().as_ref().unwrap()) {
        print!("  Rectangle: {:.1} x {:.1}\n", (*rect.lock().unwrap().as_ref().unwrap()).width, (*rect.lock().unwrap().as_ref().unwrap()).height);
    } else let (mut (*circle.lock().unwrap().as_ref().unwrap()), mut (*ok.lock().unwrap().as_ref().unwrap())) = match (*s.lock().unwrap().as_ref().unwrap()).downcast_ref::<Circle>() { Some(v) => (v.clone(), true), None => (Default::default(), false) };
    if (*ok.lock().unwrap().as_ref().unwrap()) {
        print!("  Circle: radius {:.1}\n", (*circle.lock().unwrap().as_ref().unwrap()).radius);
    }
}

fn main() {
    let mut values = std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["hello world".to_string(), 42, 3.14159, true, vec![1, 2, 3]])));
    println!("{}", "=== Processing values ===".to_string());
    for (_, val) in (*values.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        process_value(std::sync::Arc::new(std::sync::Mutex::new(Some(val))));
    }
    println!("{}", "\n=== Assertion without check ===".to_string());
    assert_without_check(std::sync::Arc::new(std::sync::Mutex::new(Some("valid string".to_string()))));
    assert_without_check(std::sync::Arc::new(std::sync::Mutex::new(Some(123))));
    println!("{}", "\n=== Interface type assertions ===".to_string());
    let mut shapes = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![Rectangle { width: 10, height: 5 }, Circle { radius: 3 }])));
    for (_, shape) in (*shapes.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        describe_shape(std::sync::Arc::new(std::sync::Mutex::new(Some(shape))));
    }
    println!("{}", "\n=== Type switch alternative ===".to_string());
    for (_, val) in (*values.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        
    }
}