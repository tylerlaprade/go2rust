#[derive(Debug)]
struct argError {
    arg: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    prob: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl argError {
    pub fn error(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*fmt.lock().unwrap().as_ref().unwrap()).sprintf(std::sync::Arc::new(std::sync::Mutex::new(Some("%d - %s".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(self.arg))), std::sync::Arc::new(std::sync::Mutex::new(Some(self.prob)))))));
    }
}

pub fn f1(arg: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    if (*arg.lock().unwrap().as_ref().unwrap()) == 42 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(-1))), std::sync::Arc::new(std::sync::Mutex::new(Some((*errors.lock().unwrap().as_ref().unwrap()).new(std::sync::Arc::new(std::sync::Mutex::new(Some("can't work with 42".to_string()))))))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*arg.lock().unwrap().as_ref().unwrap()) + 3))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

pub fn f2(arg: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    if (*arg.lock().unwrap().as_ref().unwrap()) == 42 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(-1))), std::sync::Arc::new(std::sync::Mutex::new(Some(argError { ,  }.clone()))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*arg.lock().unwrap().as_ref().unwrap()) + 3))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

fn main() {
    for (_, i) in vec![7, 42].iter().enumerate() {
        let (mut r, mut e) = f1(std::sync::Arc::new(std::sync::Mutex::new(Some(i))));
    if (*e.lock().unwrap().as_ref().unwrap()).is_some() {
        println!("{} {}", "f1 failed:".to_string(), (*e.lock().unwrap().as_ref().unwrap()));
    } else {
        println!("{} {}", "f1 worked:".to_string(), (*r.lock().unwrap().as_ref().unwrap()));
    }
    }
    for (_, i) in vec![7, 42].iter().enumerate() {
        let (mut r, mut e) = f2(std::sync::Arc::new(std::sync::Mutex::new(Some(i))));
    if (*e.lock().unwrap().as_ref().unwrap()).is_some() {
        println!("{} {}", "f2 failed:".to_string(), (*e.lock().unwrap().as_ref().unwrap()));
    } else {
        println!("{} {}", "f2 worked:".to_string(), (*r.lock().unwrap().as_ref().unwrap()));
    }
    }
    let (_, mut e) = f2(std::sync::Arc::new(std::sync::Mutex::new(Some(42))));
    let (mut ae, mut ok) = match (*e.lock().unwrap().as_ref().unwrap()).downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<argError>>>>() { Some(v) => (v.clone(), true), None => (Default::default(), false) };
    if (*ok.lock().unwrap().as_ref().unwrap()) {
        println!("{}", (*ae.lock().unwrap().as_ref().unwrap()).arg);
        println!("{}", (*ae.lock().unwrap().as_ref().unwrap()).prob);
    }
}