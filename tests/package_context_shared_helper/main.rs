#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::example_com_contextshared_core::__go_init_all();
    ::example_com_contextshared_event::__go_init_all();

    let mut ctx = example_com_contextshared_event::set_exporter(Arc::new(Mutex::new(Some(Arc::new(move |ctx: Arc<Mutex<Option<GoContext>>>| -> Arc<Mutex<Option<GoContext>>> {
        return ctx.clone();
    }) as Arc<dyn Fn(Arc<Mutex<Option<GoContext>>>) -> Arc<Mutex<Option<GoContext>>> + Send + Sync>))));
    let _ = &ctx;
    println!("{}", format!("{}", "compiled".to_string()));
}