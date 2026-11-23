#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::math::__go_init_all();
    ::math_bits::__go_init_all();

    println!("{} {} {}", format!("{}", { let __tmp_x = math::inf(Arc::new(Mutex::new(Some(1)))); let __tmp_y = 1e+308; __tmp_x > __tmp_y }), format!("{}", math::is_inf(Arc::new(Mutex::new(Some(math::inf(Arc::new(Mutex::new(Some(-1))))))), Arc::new(Mutex::new(Some(-1))))), format!("{}", math::is_na_n(Arc::new(Mutex::new(Some(math::na_n()))))));
    println!("{} {} {} {} {}", format!("{}", math::floor(Arc::new(Mutex::new(Some(-2.5))))), format!("{}", math::ceil(Arc::new(Mutex::new(Some(2.1))))), format!("{}", math::trunc(Arc::new(Mutex::new(Some(-2.7))))), format!("{}", math::round(Arc::new(Mutex::new(Some(2.5))))), format!("{}", math::abs(Arc::new(Mutex::new(Some(-3.0))))));
    println!("{} {} {} {} {} {}", format!("{}", math::sqrt(Arc::new(Mutex::new(Some(2.0))))), format!("{}", math::pow(Arc::new(Mutex::new(Some(2.0))), Arc::new(Mutex::new(Some(10.0))))), format!("{}", math::exp(Arc::new(Mutex::new(Some(1.0))))), format!("{}", math::log(Arc::new(Mutex::new(Some(math::E))))), format!("{}", math::log2(Arc::new(Mutex::new(Some(8.0))))), format!("{}", math::log10(Arc::new(Mutex::new(Some(1000.0))))));
    println!("{} {} {} {}", format!("{}", math::max(Arc::new(Mutex::new(Some(3.0))), Arc::new(Mutex::new(Some(7.0))))), format!("{}", math::min(Arc::new(Mutex::new(Some(3.0))), Arc::new(Mutex::new(Some(7.0))))), format!("{}", math::hypot(Arc::new(Mutex::new(Some(3.0))), Arc::new(Mutex::new(Some(4.0))))), format!("{}", math::r#mod(Arc::new(Mutex::new(Some(7.0))), Arc::new(Mutex::new(Some(3.0))))));
    println!("{} {} {}", format!("{}", math::float64bits(Arc::new(Mutex::new(Some(1.5))))), format!("{}", math::float64frombits(Arc::new(Mutex::new(Some(0x3ff8000000000000 as u64))))), format!("{}", math::signbit(Arc::new(Mutex::new(Some(math::copysign(Arc::new(Mutex::new(Some(0.0))), Arc::new(Mutex::new(Some(-1.0))))))))));
    println!("{} {} {} {} {}", format!("{}", math::sin(Arc::new(Mutex::new(Some(0.0))))), format!("{}", math::cos(Arc::new(Mutex::new(Some(0.0))))), format!("{}", { let __tmp_x = math::atan2(Arc::new(Mutex::new(Some(1.0))), Arc::new(Mutex::new(Some(1.0)))); let __tmp_y = 4.0; __tmp_x * __tmp_y }), format!("{}", 9223372036854775807i64), format!("{}", { let __tmp_x = (math::MAX_FLOAT64 as f64); let __tmp_y = 1.0; __tmp_x > __tmp_y }));
    let (mut frac, mut exp) = math::frexp(Arc::new(Mutex::new(Some(8.0))));
    println!("{} {} {} {}", format!("{}", frac), format!("{}", exp), format!("{}", math::ldexp(Arc::new(Mutex::new(Some(0.5))), Arc::new(Mutex::new(Some(4))))), format!("{}", math::cbrt(Arc::new(Mutex::new(Some(27.0))))));
    let (mut i, mut f) = math::modf(Arc::new(Mutex::new(Some(3.75))));
    println!("{} {}", format!("{}", i), format!("{}", f));
}