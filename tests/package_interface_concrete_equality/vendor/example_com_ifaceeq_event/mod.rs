use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn is_msg(l: Arc<Mutex<Option<example_com_ifaceeq_label::Label>>>) -> bool {
    return { let __left_holder = (*l.lock().unwrap().as_ref().unwrap()).key().clone(); let __left_guard = __left_holder.lock().unwrap(); let __left = __left_guard.as_ref().unwrap().as_ref(); let __right_holder = example_com_ifaceeq_keys::Msg.lock().unwrap().as_ref().unwrap().clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_value = __right_guard.as_ref().unwrap(); let __right: &(dyn example_com_ifaceeq_label::Key + Send + Sync) = __right_value; let __eq = __left.__go_eq_key(__right); __eq };
}