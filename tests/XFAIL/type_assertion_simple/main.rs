fn main() {
    let mut x = "hello".to_string();
    let (mut s, mut ok) = match x.downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    if ok {
        println!("{} {}", "x is string:".to_string(), s);
    }
    let mut str = match x.downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    println!("{} {}", "Asserted string:".to_string(), str);
    let (mut n, mut ok) = match x.downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if ok {
        println!("{} {}", "x is int:".to_string(), n);
    } else {
        println!("{}", "x is not an int".to_string());
    }
}