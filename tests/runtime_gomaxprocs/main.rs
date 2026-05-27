use std::sync::{Arc, Mutex};

pub mod runtime {
    use super::*;
    pub fn g_o_m_a_x_p_r_o_c_s<T0>(_arg0: T0) -> i32 {
        std::thread::available_parallelism().map(|n| n.get() as i32).unwrap_or(1).max(1)
    }
}


fn main() {
    println!("{}", format!("{}", { let __tmp_x = runtime::g_o_m_a_x_p_r_o_c_s(0); let __tmp_y = 0; __tmp_x > __tmp_y }));
}