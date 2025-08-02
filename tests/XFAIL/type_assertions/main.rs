fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
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
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
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
    fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>>;
}

#[derive(Debug)]
struct Rectangle {
    width: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
    height: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Circle {
    radius: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

impl Rectangle {
    pub fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Circle {
    pub fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

pub fn process_value(value: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>>) {
    let (mut str, mut ok) = match (*value.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("String value: {} (length: {})\n", (*str.lock().unwrap().as_mut().unwrap()), (*str.lock().unwrap().as_mut().unwrap()).len());
        return;
    }

    let (mut num, mut ok) = match (*value.lock().unwrap().as_mut().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("Integer value: {} (doubled: {})\n", (*num.lock().unwrap().as_mut().unwrap()), (*num.lock().unwrap().as_mut().unwrap()) * 2);
        return;
    }

    let (mut f, mut ok) = match (*value.lock().unwrap().as_mut().unwrap()).downcast_ref::<f64>() { Some(v) => (v.clone(), true), None => (0.0, false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("Float value: {:.2} (squared: {:.2})\n", (*f.lock().unwrap().as_mut().unwrap()), (*f.lock().unwrap().as_mut().unwrap()) * (*f.lock().unwrap().as_mut().unwrap()));
        return;
    }

    print!("Unknown type: %T with value: {}\n", (*value.lock().unwrap().as_mut().unwrap()), (*value.lock().unwrap().as_mut().unwrap()));
}

pub fn assert_without_check(value: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>>) {
    // defer () // TODO: defer not yet supported

    let mut str = std::sync::Arc::new(std::sync::Mutex::new(Some(match (*value.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) })));
    print!("Asserted string: {}\n", (*str.lock().unwrap().as_mut().unwrap()));
}

pub fn describe_shape(s: std::sync::Arc<std::sync::Mutex<Option<Box<dyn Shape>>>>) {
    print!("Shape area: {:.2}\n", (*(*s.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_mut().unwrap()));

    let (mut rect, mut ok) = match (*s.lock().unwrap().as_mut().unwrap()).downcast_ref::<Rectangle>() { Some(v) => (v.clone(), true), None => (Default::default(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("  Rectangle: {:.1} x {:.1}\n", (*rect.lock().unwrap().as_mut().unwrap()).width, (*rect.lock().unwrap().as_mut().unwrap()).height);
    } else let (mut circle, mut ok) = match (*s.lock().unwrap().as_mut().unwrap()).downcast_ref::<Circle>() { Some(v) => (v.clone(), true), None => (Default::default(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("  Circle: radius {:.1}\n", (*circle.lock().unwrap().as_mut().unwrap()).radius);
    }
}

fn main() {
    let mut values = std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["hello world".to_string(), 42, 3.14159, true, std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3])))])));

    println!("{}", "=== Processing values ===".to_string());
    for val in &(*values.lock().unwrap().as_mut().unwrap()) {
        process_value(std::sync::Arc::new(std::sync::Mutex::new(Some(val))));
    }

    println!("{}", "\n=== Assertion without check ===".to_string());
    assert_without_check(std::sync::Arc::new(std::sync::Mutex::new(Some("valid string".to_string()))));
    assert_without_check(std::sync::Arc::new(std::sync::Mutex::new(Some(123))));

    println!("{}", "\n=== Interface type assertions ===".to_string());
    let mut shapes = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![Rectangle { width: std::sync::Arc::new(std::sync::Mutex::new(Some(10))), height: std::sync::Arc::new(std::sync::Mutex::new(Some(5))) }, Circle { radius: std::sync::Arc::new(std::sync::Mutex::new(Some(3))) }])));

    for shape in &(*shapes.lock().unwrap().as_mut().unwrap()) {
        describe_shape(std::sync::Arc::new(std::sync::Mutex::new(Some(shape))));
    }

    println!("{}", "\n=== Type switch alternative ===".to_string());
    for val in &(*values.lock().unwrap().as_mut().unwrap()) {
        // TODO: Unhandled statement type: TypeSwitchStmt
    }
}