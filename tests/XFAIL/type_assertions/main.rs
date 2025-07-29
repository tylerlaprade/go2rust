pub fn process_value(value: Box<dyn std::any::Any>) {
    let (mut str, mut ok) = ;
    if ok {
        print!("String value: {} (length: {})\n", str, str.len());
        return;
    }
    let (mut num, mut ok) = ;
    if ok {
        print!("Integer value: {} (doubled: {})\n", num, num * 2);
        return;
    }
    let (mut f, mut ok) = ;
    if ok {
        print!("Float value: {:.2} (squared: {:.2})\n", f, f * f);
        return;
    }
    print!("Unknown type: %T with value: {}\n", value, value);
}

pub fn assert_without_check(value: Box<dyn std::any::Any>) {
    
    let mut str = ;
    print!("Asserted string: {}\n", str);
}



#[derive(Debug)]
struct Rectangle {
    width: f64,
}

pub fn area() -> f64 {

    return r.width * r.height;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

pub fn area() -> f64 {

    return 3.14159 * c.radius * c.radius;
}

pub fn describe_shape(s: Shape) {
    print!("Shape area: {:.2}\n", s.area());
    let (mut rect, mut ok) = ;
    if ok {
        print!("  Rectangle: {:.1} x {:.1}\n", rect.width, rect.height);
    } else let (mut circle, mut ok) = ;
    if ok {
        print!("  Circle: radius {:.1}\n", circle.radius);
    }
}

fn main() {
    let mut values = vec!["hello world".to_string(), 42, 3.14159, true, vec![1, 2, 3]];
    println!("{}", "=== Processing values ===".to_string());
    for (_, val) in values.iter().enumerate() {
        process_value(val);
    }
    println!("{}", "\n=== Assertion without check ===".to_string());
    assert_without_check("valid string".to_string());
    assert_without_check(123);
    println!("{}", "\n=== Interface type assertions ===".to_string());
    let mut shapes = vec![Rectangle { width: 10, height: 5 }, Circle { radius: 3 }];
    for (_, shape) in shapes.iter().enumerate() {
        describe_shape(shape);
    }
    println!("{}", "\n=== Type switch alternative ===".to_string());
    for (_, val) in values.iter().enumerate() {
        
    }
}