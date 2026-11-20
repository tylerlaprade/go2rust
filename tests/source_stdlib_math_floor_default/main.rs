use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::sync::{Arc, Mutex};

fn main() {
    ::math::__go_init_all();
    ::math_bits::__go_init_all();

    if { let __tmp_x = math::floor(Arc::new(Mutex::new(Some(2.6)))); let __tmp_y = 2.0; __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("math.Floor mismatch".to_string()) as Box<dyn Any + Send + Sync>);
    }
    {
            let __go_print_arg_0 = format!("{}", 1);
            eprintln!("{}", __go_print_arg_0)
        };
}