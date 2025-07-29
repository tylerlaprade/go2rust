fn main() {
    let mut p;
    if p.is_none() {
        println!("{}", "p is nil".to_string());
    }
    let mut q = None;
    if q.is_none() {
        println!("{}", "q is nil".to_string());
    }
    let mut x = 42;
    p = &x;
    if p.is_some() {
        println!("{} {}", "p is not nil, value:".to_string(), );
    }
    p = None;
    if p.is_none() {
        println!("{}", "p is nil again".to_string());
    }
}