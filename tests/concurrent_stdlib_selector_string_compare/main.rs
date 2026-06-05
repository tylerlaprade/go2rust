use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn has_name(e: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, name: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    if false {
        let mut ch = GoChannel::<bool>::new_buffered(1 as usize);
        let ch_thread = ch.clone(); std::thread::spawn(move || {
        ch_thread.send(has_name(Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { ..Default::default() }))), Arc::new(Mutex::new(Some("x".to_string())))));;;
    });
        println!("{}", format!("{}", ch.recv().unwrap_or_default()));
    }
    println!("{}", format!("{}", "ok".to_string()));
}