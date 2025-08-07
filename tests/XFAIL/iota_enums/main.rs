use std::sync::{Arc, Mutex};

const RED: Color = 0;
const GREEN: i32 = 1;
const BLUE: i32 = 2;
const YELLOW: i32 = 3;


#[derive(Debug, Clone)]
struct Color(Arc<Mutex<Option<i32>>>);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}


fn main() {
    println!("{} {}", "Red:".to_string(), RED);
    println!("{} {}", "Green:".to_string(), GREEN);
    println!("{} {}", "Blue:".to_string(), BLUE);
    println!("{} {}", "Yellow:".to_string(), YELLOW);
}