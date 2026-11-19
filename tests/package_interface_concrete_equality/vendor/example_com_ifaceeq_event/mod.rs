use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub fn is_msg(l: Arc<Mutex<Option<example_com_ifaceeq_label::r#mod::Label>>>) -> bool {
    return { let __left_holder = (*l.lock().unwrap().as_ref().unwrap()).key().clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn example_com_ifaceeq_label::r#mod::Key + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = example_com_ifaceeq_keys::r#mod::String_Ptr({ let __arg_holder = example_com_ifaceeq_keys::Msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __right_opt: Option<&(dyn example_com_ifaceeq_label::r#mod::Key + Send + Sync)> = Some(&__right_wrapper as &(dyn example_com_ifaceeq_label::r#mod::Key + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_key(__right), _ => false }; __eq };
}