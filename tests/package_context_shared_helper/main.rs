use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    let mut ctx = example_com_contextshared_event::set_exporter(Arc::new(Mutex::new(Some(Box::new(move |ctx: Arc<Mutex<Option<GoContext>>>| -> Arc<Mutex<Option<GoContext>>> {
        return ctx.clone();
    }) as Box<dyn FnMut(Arc<Mutex<Option<GoContext>>>) -> Arc<Mutex<Option<GoContext>>> + Send + Sync>))));
    let _ = { let __v = (*ctx.lock().unwrap().as_ref().unwrap()).clone(); __v };
    println!("{}", format!("{}", "compiled".to_string()));
}