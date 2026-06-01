mod aaa;
mod bbb;
use aaa::*;
use bbb::*;

fn main() {
    __go_init_all();

    println!("{} {} {}", format!("{}", { let __v = (*A.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*B.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*C.borrow().as_ref().unwrap()).clone(); __v }));
}

pub(crate) fn __go_init_all() {
    aaa::__go_zero_globals();
    bbb::__go_zero_globals();
    aaa::__go_init_order_0();
    bbb::__go_init_order_1();
    aaa::__go_init_order_2();
}
