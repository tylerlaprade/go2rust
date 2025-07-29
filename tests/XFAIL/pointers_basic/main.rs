#[derive(Debug)]
struct Point {
    x: i32,
}

fn main() {
    let mut x = 42;
    let mut p = &x;
    println!("{} {}", "Value of x:".to_string(), x);
    println!("{} {}", "Address of x:".to_string(), p);
    println!("{} {}", "Value through pointer:".to_string(), );
     = 100;
    println!("{} {}", "Modified x:".to_string(), x);
    let mut point = &Point { x: 10, y: 20 };
    println!("{} {}", "Point:".to_string(), point);
    println!("{} {}", "Point X:".to_string(), point.x);
    println!("{} {}", "Point Y:".to_string(), point.y);
    point.x = 30;
    point.y = 40;
    println!("{} {}", "Modified point:".to_string(), point);
    let mut q = p;
     = 200;
    println!("{} {}", "x after modifying through q:".to_string(), x);
    let mut newPoint = new(POINT);
    newPoint.x = 5;
    newPoint.y = 15;
    println!("{} {}", "New point:".to_string(), newPoint);
}