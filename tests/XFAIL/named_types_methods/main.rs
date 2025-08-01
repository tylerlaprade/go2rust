





impl Celsius {
    pub fn to_fahrenheit(&self) -> std::sync::Arc<std::sync::Mutex<Option<Fahrenheit>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(fahrenheit(std::sync::Arc::new(std::sync::Mutex::new(Some(self * 9 / 5 + 32)))))));
    }
}

impl Fahrenheit {
    pub fn to_celsius(&self) -> std::sync::Arc<std::sync::Mutex<Option<Celsius>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(celsius(std::sync::Arc::new(std::sync::Mutex::new(Some((self - 32) * 5 / 9)))))));
    }
}

fn main() {
    let mut temp: std::sync::Arc<std::sync::Mutex<Option<Celsius>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(100)));
    print!("{}°C = {}°F\n", (*temp.lock().unwrap().as_mut().unwrap()), (*temp.lock().unwrap().as_mut().unwrap()).to_fahrenheit());
    let mut f: std::sync::Arc<std::sync::Mutex<Option<Fahrenheit>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(212)));
    print!("{}°F = {}°C\n", (*f.lock().unwrap().as_mut().unwrap()), (*f.lock().unwrap().as_mut().unwrap()).to_celsius());
    let mut s: std::sync::Arc<std::sync::Mutex<Option<StringAlias>>> = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    println!("{}", (*s.lock().unwrap().as_mut().unwrap()));
}